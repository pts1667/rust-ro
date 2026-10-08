import unittest

from rathena_script.codegen import BackLoop, Goto, Label, walk, wrap_backward_labels
from rathena_script.parser import parse_body


def nested_body():
    """`L_Retry` sits inside two switches and only a `goto` after it jumps back to it, like `#Sneak`."""
    return parse_body("""
        switch (select("a:b")) {
        case 1:
            switch (select("c:d")) {
            case 1:
                mes "before";
        L_Retry:
                input @password;
                if (@password != 1) goto L_Retry;
                close;
            case 2:
                close;
            }
        }
        end;
    """)


class ConvertNpcTests(unittest.TestCase):
    def test_a_stray_closing_parenthesis_ends_an_assignment_like_rathena(self):
        statements = parse_body(".@total = ( .@a <= ( 1 ) ) ? .@a : ( 2 ));")
        self.assertEqual(len(statements), 1)

    def test_a_label_that_only_a_later_goto_jumps_back_to_becomes_a_loop(self):
        loops = loops_in(wrap_backward_labels(nested_body()))
        self.assertEqual([loop.name for loop in loops], ["L_Retry"])
        self.assertTrue(any(isinstance(node, Goto) for node in walk(loops[0].body)))
        self.assertFalse(any(isinstance(item, Label) for item in loops[0].body))

    def test_a_label_that_something_else_jumps_to_stays_a_label(self):
        statements = parse_body("""
            if (1) { goto L_Elsewhere; }
            switch (select("a:b")) {
            case 1:
        L_Elsewhere:
                mes "x";
                goto L_Elsewhere;
            }
        """)
        self.assertEqual(loops_in(wrap_backward_labels(statements)), [])


def loops_in(statements):
    return [node for node in walk(statements) if isinstance(node, BackLoop)]


if __name__ == "__main__":
    unittest.main()
