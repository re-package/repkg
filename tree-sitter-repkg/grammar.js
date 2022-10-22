module.exports = grammar({
    name: 'repkg',

    extras: $ => [
        $.comment,
        /[\s\p{Zs}\uFEFF\u2060\u200B]/,
    ],

    rules: {
        source_file: $ => $._body,

        _body: $ => seq(
            $._expr,
            repeat($._expr),
        ),

        _expr: $ => choice(
            seq($.command, $._newline),
            $.namespace,
            $.import_expr,
            $.variable_def,
        ),

        variable_def: $ => seq(
            $._identifier_without_ver,
            token('='),
            $._primitive,
        ),

        import_expr: $ => seq(
            'import',
            $._path,
            optional(seq(
                $._dot,
                $.wildcard
            ))
        ),

        namespace: $ => seq(
            field('namespace_name', $.identifier),
            '{',
            $._body,
            '}',
        ),

        command: $ => seq(
            field('program', $._path),
            field('args', repeat(
                choice(
                    seq(
                        '(',
                        $.command,
                        ')',
                    ),
                    $.variable,
                    $.text,
                )
            )),
        ),

        _path: $ => prec(4, choice(
            $.identifier,
            $.nested_identifier,
        )),

        identifier: $ => prec(2, choice(
            $._identifier_with_ver,
            $._identifier_without_ver,
        )),

        _dot: $ => prec(50, token('.')),

        nested_identifier: $ => prec(5, seq(
            field('path', $._path),
            $._dot,
            field('name', $.identifier),
        )),

        variable: $ => prec(2, seq(
            '%',
            field('var', choice($._identifier_without_ver, $.number, '%')),
        )),

        _identifier_without_ver: $ => /[-a-z_A-Z]+/,

        _identifier_with_ver: $ => prec.right(seq(
            $._identifier_without_ver,
            '@',
            $._identifier_without_ver,
        )),

        _primitive: $ => choice(
            $.number,
            $.string,
            $.array,
        ),

        text: $ => /[^ \n\r()%][^ \n\r()]*/,

        number: $ => /[0-9]+/,

        string: $ => choice(
            seq(
                '"',
                repeat(/[^"]/),
                '"',
            ),
            seq(
                '\'',
                repeat(/[^']/),
                '\''
            ),
        ),

        array: $ => seq(
            '[',
            optional(
                seq(
                    $._primitive,
                    repeat(seq(
                        ',',
                        $._primitive,
                    )),
                    optional(','),
                )
            ),
            ']',
        ),

        wildcard: $ => token('*'),

        _newline: $ => choice(
            '\n',
            '\r',
            '\n\r',
            '\0',
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
