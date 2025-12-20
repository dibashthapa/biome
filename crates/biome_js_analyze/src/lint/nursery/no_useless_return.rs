use biome_analyze::{
    Ast, QueryMatch, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    JsBlockStatement, JsCaseClause, JsFunctionBody, JsIfStatement, JsReturnStatement,
    JsStatementList, JsSwitchCaseList, JsTryFinallyStatement, JsTryStatement,
};
use biome_rowan::AstNodeList;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_useless_return::NoUselessReturnOptions;

declare_lint_rule! {
    /// Disallow redundant return statements.
    ///
    /// A return statement that doesn't change control flow is redundant, and can be removed
    /// without changing the behavior of the program.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function foo() {
    ///     return;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function bar() {
    ///     if (x) {
    ///         doSomething();
    ///         return;
    ///     } else {
    ///         doSomethingElse();
    ///     }
    ///     return;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function baz() {
    ///     if (x) {
    ///         return;
    ///     }
    ///     return;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function foo() {
    ///     return 5;
    /// }
    /// ```
    ///
    /// ```js
    /// function bar() {
    ///     if (x) {
    ///         return;
    ///     }
    ///     doSomething();
    /// }
    /// ```
    ///
    /// ```js
    /// function baz() {
    ///     for (let i = 0; i < 10; i++) {
    ///         if (condition) {
    ///             return;
    ///         }
    ///     }
    /// }
    /// ```
    ///
    pub NoUselessReturn {
        version: "next",
        name: "noUselessReturn",
        language: "js",
        sources: &[RuleSource::Eslint("no-useless-return").same()],
        recommended: false,
    }
}

impl Rule for NoUselessReturn {
    type Query = Ast<JsReturnStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoUselessReturnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let ret = ctx.query();

        if ret.argument().is_some() {
            return None;
        }

        let result = analyze_ancestors(&ret)?;
        dbg!(&result);

        match result {
            AncestorAnalysis::NotUseless | AncestorAnalysis::NotAtFunctionEnd => None,
            AncestorAnalysis::AtFunctionEnd => {
                return if has_subsequent_statements(ret) {
                    None
                } else {
                    Some(())
                };
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let query = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                query.range(),
                markup! {
                    "Unnecessary return statement."
                },
            )
            .note(markup! {
                "This return statement can be removed without changing the behavior of the program."
            }),
        )
    }
}

#[derive(Debug)]
enum AncestorAnalysis {
    NotUseless,
    AtFunctionEnd,
    NotAtFunctionEnd,
}

// Check if a return statement is useless
// A return is useless if:
// 1. It has no value
// 2. It's at the end of the function (last statement in all enclosing blocks)
// 3. There's no reachable code after it that would execute if removed
// 4. It's not inside a loop (where it serves as early exit)
// 5. It's not inside a finally block (where it overrides other returns)
// 6. It's not preventing switch fallthrough
//

fn analyze_ancestors(ret: &JsReturnStatement) -> Option<AncestorAnalysis> {
    use biome_js_syntax::JsSyntaxKind;

    let ret_syntax = ret.syntax();

    let mut current_range = ret_syntax.text_range();

    for ancestor in ret_syntax.ancestors() {
        // dbg!(&ancestor.kind());
        match ancestor.kind() {
            JsSyntaxKind::JS_FUNCTION_BODY => {
                let body = JsFunctionBody::cast(ancestor)?;

                return if is_range_in_last_statement(&body.statements(), current_range) {
                    Some(AncestorAnalysis::AtFunctionEnd)
                } else {
                    Some(AncestorAnalysis::NotAtFunctionEnd)
                };
            }
            JsSyntaxKind::JS_FUNCTION_DECLARATION | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                return Some(AncestorAnalysis::NotAtFunctionEnd);
            }

            JsSyntaxKind::JS_FOR_STATEMENT
            | JsSyntaxKind::JS_FOR_IN_STATEMENT
            | JsSyntaxKind::JS_FOR_OF_STATEMENT
            | JsSyntaxKind::JS_WHILE_STATEMENT
            | JsSyntaxKind::JS_DO_WHILE_STATEMENT => return Some(AncestorAnalysis::NotUseless),

            JsSyntaxKind::JS_BLOCK_STATEMENT => {
                let block = JsBlockStatement::cast_ref(&ancestor)?;
                let parent = ancestor.parent()?;

                if parent.kind() == JsSyntaxKind::JS_TRY_FINALLY_STATEMENT {
                    let try_stmt = JsTryFinallyStatement::cast(parent)?;

                    if try_stmt
                        .finally_clause()
                        .ok()
                        .is_some_and(|f| f.range().contains_range(block.range()))
                    {
                        return Some(AncestorAnalysis::NotUseless);
                    }

                    if !is_range_in_last_statement(&block.statements(), current_range) {
                        return Some(AncestorAnalysis::NotAtFunctionEnd);
                    }
                    current_range = block.range();
                }
            }

            JsSyntaxKind::JS_CASE_CLAUSE => {
                let case = JsCaseClause::cast_ref(&ancestor)?;
                let parent = ancestor.parent()?;

                if parent.kind() == JsSyntaxKind::JS_SWITCH_CASE_LIST {
                    let cases = JsSwitchCaseList::cast(parent)?;

                    if let Some((idx, _)) = cases
                        .iter()
                        .enumerate()
                        .find(|(_, c)| c.range() == case.range())
                    {
                        let is_last_case = idx == cases.len() - 1;

                        // let last_range = &case.consequent().last()?;
                        // dbg!(&last_range, &is_last_case, &current_range);

                        if !is_last_case
                            && case.consequent().last().is_some_and(|last_stmt| {
                                last_stmt.range().contains_range(current_range)
                            })
                        {
                            let subsequent_cases_empty = cases
                                .iter()
                                .skip(idx + 1)
                                .all(|c| c.consequent().is_empty());

                            if !subsequent_cases_empty {
                                return Some(AncestorAnalysis::NotUseless);
                            }
                        }
                    }
                }
                current_range = case.range();
            }

            JsSyntaxKind::JS_IF_STATEMENT => {
                let if_stmt = JsIfStatement::cast_ref(&ancestor)?;
                let consequent_stmt = if_stmt.consequent().ok()?;

                let in_consequent = consequent_stmt.range().contains_range(current_range);

                let in_alternate = if_stmt
                    .else_clause()
                    .is_some_and(|alt| alt.range().contains_range(current_range));

                if in_consequent || in_alternate {
                    current_range = if_stmt.range();
                }
            }

            JsSyntaxKind::JS_TRY_STATEMENT => {
                let try_stmt = JsTryStatement::cast_ref(&ancestor)?;

                let in_try = try_stmt.body().ok()?.range().contains_range(current_range);

                let in_catch = try_stmt
                    .catch_clause()
                    .ok()?
                    .range()
                    .contains_range(current_range);

                if in_try || in_catch {
                    current_range = try_stmt.range();
                }
            }

            _ => {
                current_range = ancestor.text_range();
            }
        }
    }

    Some(AncestorAnalysis::NotAtFunctionEnd)
}

fn is_range_in_last_statement(statements: &JsStatementList, range: TextRange) -> bool {
    statements
        .last()
        .is_some_and(|last| last.range().contains_range(range))
}

fn has_subsequent_statements(ret: &JsReturnStatement) -> bool {
    use biome_js_syntax::JsSyntaxKind;

    let Some(parent) = ret.syntax().parent() else {
        return false;
    };

    if parent.kind() != JsSyntaxKind::JS_BLOCK_STATEMENT {
        return false;
    }

    let Some(block) = JsBlockStatement::cast(parent) else {
        return false;
    };
    let statements = block.statements();

    // Find our return and check if there are non-empty statements after
    let return_range = ret.range();
    let mut found_return = false;

    for stmt in statements {
        if found_return {
            // Any statement after the return makes the return non-useless
            if !matches!(stmt, biome_js_syntax::AnyJsStatement::JsEmptyStatement(_)) {
                return true;
            }
        } else if stmt.range().contains_range(return_range) {
            found_return = true;
        }
    }

    false
}
