module.exports = grammar({
    name: 'repkg',

    word: $ => $.identifier,

    extras: $ => [
        $.comment,
        /[\s\p{Zs}\uFEFF\u2060\u200B]/,
    ],

    rules: {
        source_file: $ => $.object,

        object: $ => prec(1, seq(
            '{',
            field('child', repeat($.definition)),
            '}',
        )),

        definition: $ => seq(
            field('id', $.identifier),
            ':',
            field('expr', choice($._expr, $.func_def)),
        ),

        func_def: $ => seq(
            optional(seq('(', ')')),
            '{',
            field('commands', repeat($.command)),
            optional(field('return', $._expr)),
            '}',
        ),

        _expr: $ => choice(
            $.object,
            $.number,
            $.string,
        ),

        command: $ => seq(
            '$',
            field('path', choice($.identifier, $.nested_identifier)),
            optional(seq(
                '(',
                field('args', repeat($.text)),
                ')',
            )),
        ),

        number: $ => /[0-9]+/,
        string: $ => choice(
            seq(
                '"',
                repeat(choice(
                    alias($.unescaped_double_string_fragment, $.string_fragment),
                )),
                '"'
            ),
            seq(
                "'",
                repeat(choice(
                    alias($.unescaped_single_string_fragment, $.string_fragment),
                )),
                "'"
            )
        ),
        unescaped_double_string_fragment: $ =>
            token.immediate(prec(1, /[^"\\]+/)),
        unescaped_single_string_fragment: $ =>
            token.immediate(prec(1, /[^'\\]+/)),
        text: $ => /[^ )]+/,
        identifier: $ => /[a-zA-Z]+/,
        nested_identifier: $ => seq(
            field('path', choice($.identifier, $.nested_identifier)),
            '.',
            field('name', $.identifier),
        ),

        comment: $ => token(choice(
            seq('//', /.*/),
            seq(
                '/*',
                /[^*]*\*+([^/*][^*]*\*+)*/,
                '/'
            )
        )),
    }
});