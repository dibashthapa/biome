/* should not generate diagnostics */
{
	function foo() {
		return 5;
	}
}
{
	function foo() {
		return null;
	}
}
{
	function foo() {
		return doSomething();
	}
}
{
	function foo() {
		if (bar) {
			doSomething();
			return;
		} else {
			doSomethingElse();
		}
		qux();
	}
}
{
	function foo() {
		switch (bar) {
			case 1:
				doSomething();
				return;
			default:
				doSomethingElse();
		}
	}
}
{
	function foo() {
		switch (bar) {
			default:
				doSomething();
				return;
			case 1:
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
					return;
				}
			default:
				doSomethingElse();
		}
	}
}
{
	function foo() {
		for (var foo = 0; foo < 10; foo++) {
			return;
		}
	}
}
{
	function foo() {
		for (var foo in bar) {
			return;
		}
	}
}
{
	function foo() {
		try {
			return 5;
		} finally {
			return; // This is allowed because it can override the returned value of 5
		}
	}
}
{
	function foo() {
		try {
			bar();
			return;
		} catch (err) {}
		baz();
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
		baz();
	}
}
{
	function foo() {
		return;
		doSomething();
	}
}
{
	function foo() {
		if (bar) return;
		return baz;
	}
}
{
	function foo() {
		if (bar) {
			return;
		}
		return baz;
	}
}
{
	function foo() {
		if (bar) baz();
		else return;
		return 5;
	}
}
{
	function foo() {
		return;
		while (foo) return;
		foo;
	}
}
{
	try {
		throw new Error('foo');
		while (false);
	} catch (err) {}
}
{
	function foo(arg) {
		throw new Error('Debugging...');
		if (!arg) {
			return;
		}
		console.log(arg);
	}
}
{
	function foo() {
		try {
			bar();
			return;
		} finally {
			baz();
		}
		qux();
	}
}
