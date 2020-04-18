import std.stdio;
import rlib;


void main()
{
	auto test = new Test();
	auto message = test.get_message();
	writeln(message);
	test.set_message("Hello from D!");
}
