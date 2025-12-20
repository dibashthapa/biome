/* should generate diagnostics */
{
	function foo() {
		return;
	}
}
{
	function foo() {
		doSomething();
		return;
	}
}
{
	function foo() {
		if (condition) {
			bar();
			return;
		} else {
			baz();
		}
	}
}
{
	function foo() {
		if (foo) return;
	}
}
{
	function foo() {
		bar();
		return /**/;
	}
}
{
	function foo() {
		bar();
		return; //
	}
}
{
	function foo() {
		if (foo) {
			return;
		}
		return;
	}
}
{
	function foo() {
		switch (bar) {
			case 1:
				doSomething();
			default:
				doSomethingElse();
				return;
		}
	}
}
{
	function foo() {
		switch (bar) {
			default:
				doSomething();
			case 1:
				doSomething();
				return;
		}
	}
}
{
	function foo() {
		switch (bar) {
			case 1:
				if (a) {
					doSomething();
					return;
				}
				break;
			default:
				doSomethingElse();
		}
	}
}
{
	function foo() {
		switch (bar) {
			case 1:
				if (a) {
					doSomething();
					return;
				} else {
					doSomething();
				}
				break;
			default:
				doSomethingElse();
		}
	}
}
{
	function foo() {
		switch (bar) {
			case 1:
				if (a) {
					doSomething();
					return;
				}
			default:
		}
	}
}
{
	function foo() {
		try {
		} catch (err) {
			return;
		}
	}
}
{
	function foo() {
		try {
			foo();
			return;
		} catch (err) {
			return 5;
		}
	}
}
{
	function foo() {
		if (something) {
			try {
				bar();
				return;
			} catch (err) {}
		}
	}
}
{
	function foo() {
		try {
			return;
		} catch (err) {
			foo();
		}
	}
}
{
	function foo() {
		try {
			return;
		} finally {
			bar();
		}
	}
}
{
	function foo() {
		try {
			bar();
		} catch (e) {
			try {
				baz();
				return;
			} catch (e) {
				qux();
			}
		}
	}
}
{
	function foo() {
		try {
		} finally {
		}
		return;
	}
}
{
	function foo() {
		try {
			return 5;
		} finally {
			function bar() {
				return;
			}
		}
	}
}
{
	function foo() {
		return;
		return;
	}
}
