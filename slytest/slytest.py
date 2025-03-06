from sly import Lexer


class FoamLexer(Lexer):
    tokens = {
        DICT_START,
        DICT_END,
        LIST_START,
        LIST_END,
        END,
        NUMBER,
        QUOTED_STRING,
        IDENTIFIER,
    }

    ignore = " \t\n"

    DICT_START = r"{"
    DICT_END = r"}"

    LIST_START = r"\("
    LIST_END = r"\)"

    END = ";"

    NUMBER = r"[0-9.]+"
    QUOTED_STRING = r'".*"'
    IDENTIFIER = r"[a-zA-Z][a-zA-Z0-9]*"


if __name__ == "__main__":
    EXAMPLE = """a 1;
dict
{
    b 2;
    c ( 1 2 3 );
    d "magic string with spaces";
}
"""

    lexer = FoamLexer()
    for elem in lexer.tokenize(EXAMPLE):
        print(elem)
