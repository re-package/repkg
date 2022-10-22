#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 76
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 48
#define ALIAS_COUNT 0
#define TOKEN_COUNT 26
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 6
#define MAX_ALIAS_SEQUENCE_LENGTH 5
#define PRODUCTION_ID_COUNT 6

enum {
  anon_sym_EQ = 1,
  anon_sym_import = 2,
  anon_sym_LBRACE = 3,
  anon_sym_RBRACE = 4,
  anon_sym_LPAREN = 5,
  anon_sym_RPAREN = 6,
  anon_sym_DOT = 7,
  anon_sym_PERCENT = 8,
  sym__identifier_without_ver = 9,
  anon_sym_AT = 10,
  sym_text = 11,
  sym_number = 12,
  anon_sym_DQUOTE = 13,
  aux_sym_string_token1 = 14,
  anon_sym_SQUOTE = 15,
  aux_sym_string_token2 = 16,
  anon_sym_LBRACK = 17,
  anon_sym_COMMA = 18,
  anon_sym_RBRACK = 19,
  sym_wildcard = 20,
  anon_sym_LF = 21,
  anon_sym_CR = 22,
  anon_sym_LF_CR = 23,
  anon_sym_ = 24,
  sym_comment = 25,
  sym_source_file = 26,
  sym__body = 27,
  sym__expr = 28,
  sym_variable_def = 29,
  sym_import_expr = 30,
  sym_namespace = 31,
  sym_command = 32,
  sym__path = 33,
  sym_identifier = 34,
  sym__dot = 35,
  sym_nested_identifier = 36,
  sym_variable = 37,
  sym__identifier_with_ver = 38,
  sym__primitive = 39,
  sym_string = 40,
  sym_array = 41,
  sym__newline = 42,
  aux_sym__body_repeat1 = 43,
  aux_sym_command_repeat1 = 44,
  aux_sym_string_repeat1 = 45,
  aux_sym_string_repeat2 = 46,
  aux_sym_array_repeat1 = 47,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_EQ] = "=",
  [anon_sym_import] = "import",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_DOT] = ".",
  [anon_sym_PERCENT] = "%",
  [sym__identifier_without_ver] = "_identifier_without_ver",
  [anon_sym_AT] = "@",
  [sym_text] = "text",
  [sym_number] = "number",
  [anon_sym_DQUOTE] = "\"",
  [aux_sym_string_token1] = "string_token1",
  [anon_sym_SQUOTE] = "'",
  [aux_sym_string_token2] = "string_token2",
  [anon_sym_LBRACK] = "[",
  [anon_sym_COMMA] = ",",
  [anon_sym_RBRACK] = "]",
  [sym_wildcard] = "wildcard",
  [anon_sym_LF] = "\n",
  [anon_sym_CR] = "\r",
  [anon_sym_LF_CR] = "\n\r",
  [anon_sym_] = " ",
  [sym_comment] = "comment",
  [sym_source_file] = "source_file",
  [sym__body] = "_body",
  [sym__expr] = "_expr",
  [sym_variable_def] = "variable_def",
  [sym_import_expr] = "import_expr",
  [sym_namespace] = "namespace",
  [sym_command] = "command",
  [sym__path] = "_path",
  [sym_identifier] = "identifier",
  [sym__dot] = "_dot",
  [sym_nested_identifier] = "nested_identifier",
  [sym_variable] = "variable",
  [sym__identifier_with_ver] = "_identifier_with_ver",
  [sym__primitive] = "_primitive",
  [sym_string] = "string",
  [sym_array] = "array",
  [sym__newline] = "_newline",
  [aux_sym__body_repeat1] = "_body_repeat1",
  [aux_sym_command_repeat1] = "command_repeat1",
  [aux_sym_string_repeat1] = "string_repeat1",
  [aux_sym_string_repeat2] = "string_repeat2",
  [aux_sym_array_repeat1] = "array_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_import] = anon_sym_import,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_PERCENT] = anon_sym_PERCENT,
  [sym__identifier_without_ver] = sym__identifier_without_ver,
  [anon_sym_AT] = anon_sym_AT,
  [sym_text] = sym_text,
  [sym_number] = sym_number,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [aux_sym_string_token1] = aux_sym_string_token1,
  [anon_sym_SQUOTE] = anon_sym_SQUOTE,
  [aux_sym_string_token2] = aux_sym_string_token2,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [sym_wildcard] = sym_wildcard,
  [anon_sym_LF] = anon_sym_LF,
  [anon_sym_CR] = anon_sym_CR,
  [anon_sym_LF_CR] = anon_sym_LF_CR,
  [anon_sym_] = anon_sym_,
  [sym_comment] = sym_comment,
  [sym_source_file] = sym_source_file,
  [sym__body] = sym__body,
  [sym__expr] = sym__expr,
  [sym_variable_def] = sym_variable_def,
  [sym_import_expr] = sym_import_expr,
  [sym_namespace] = sym_namespace,
  [sym_command] = sym_command,
  [sym__path] = sym__path,
  [sym_identifier] = sym_identifier,
  [sym__dot] = sym__dot,
  [sym_nested_identifier] = sym_nested_identifier,
  [sym_variable] = sym_variable,
  [sym__identifier_with_ver] = sym__identifier_with_ver,
  [sym__primitive] = sym__primitive,
  [sym_string] = sym_string,
  [sym_array] = sym_array,
  [sym__newline] = sym__newline,
  [aux_sym__body_repeat1] = aux_sym__body_repeat1,
  [aux_sym_command_repeat1] = aux_sym_command_repeat1,
  [aux_sym_string_repeat1] = aux_sym_string_repeat1,
  [aux_sym_string_repeat2] = aux_sym_string_repeat2,
  [aux_sym_array_repeat1] = aux_sym_array_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_import] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PERCENT] = {
    .visible = true,
    .named = false,
  },
  [sym__identifier_without_ver] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_AT] = {
    .visible = true,
    .named = false,
  },
  [sym_text] = {
    .visible = true,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_string_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_SQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_string_token2] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [sym_wildcard] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LF] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_CR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LF_CR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_] = {
    .visible = true,
    .named = false,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym__body] = {
    .visible = false,
    .named = true,
  },
  [sym__expr] = {
    .visible = false,
    .named = true,
  },
  [sym_variable_def] = {
    .visible = true,
    .named = true,
  },
  [sym_import_expr] = {
    .visible = true,
    .named = true,
  },
  [sym_namespace] = {
    .visible = true,
    .named = true,
  },
  [sym_command] = {
    .visible = true,
    .named = true,
  },
  [sym__path] = {
    .visible = false,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym__dot] = {
    .visible = false,
    .named = true,
  },
  [sym_nested_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_variable] = {
    .visible = true,
    .named = true,
  },
  [sym__identifier_with_ver] = {
    .visible = false,
    .named = true,
  },
  [sym__primitive] = {
    .visible = false,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_array] = {
    .visible = true,
    .named = true,
  },
  [sym__newline] = {
    .visible = false,
    .named = true,
  },
  [aux_sym__body_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_command_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_string_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_string_repeat2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_array_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_args = 1,
  field_name = 2,
  field_namespace_name = 3,
  field_path = 4,
  field_program = 5,
  field_var = 6,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_args] = "args",
  [field_name] = "name",
  [field_namespace_name] = "namespace_name",
  [field_path] = "path",
  [field_program] = "program",
  [field_var] = "var",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 2},
  [3] = {.index = 3, .length = 1},
  [4] = {.index = 4, .length = 2},
  [5] = {.index = 6, .length = 1},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_program, 0},
  [1] =
    {field_args, 1},
    {field_program, 0},
  [3] =
    {field_var, 1},
  [4] =
    {field_name, 2},
    {field_path, 0},
  [6] =
    {field_namespace_name, 0},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 6,
  [15] = 15,
  [16] = 16,
  [17] = 8,
  [18] = 11,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 12,
  [26] = 26,
  [27] = 13,
  [28] = 12,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 29,
  [35] = 35,
  [36] = 10,
  [37] = 37,
  [38] = 11,
  [39] = 15,
  [40] = 15,
  [41] = 11,
  [42] = 42,
  [43] = 8,
  [44] = 8,
  [45] = 45,
  [46] = 22,
  [47] = 47,
  [48] = 48,
  [49] = 24,
  [50] = 50,
  [51] = 51,
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 55,
  [56] = 56,
  [57] = 57,
  [58] = 58,
  [59] = 54,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 53,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 67,
  [68] = 68,
  [69] = 68,
  [70] = 70,
  [71] = 71,
  [72] = 67,
  [73] = 68,
  [74] = 68,
  [75] = 75,
};

static inline bool anon_sym__character_set_1(int32_t c) {
  return (c < 8192
    ? (c < ' '
      ? (c < '\r'
        ? (c >= '\t' && c <= '\n')
        : c <= '\r')
      : (c <= ' ' || (c < 5760
        ? c == 160
        : c <= 5760)))
    : (c <= 8203 || (c < 12288
      ? (c < 8287
        ? c == 8239
        : c <= 8288)
      : (c <= 12288 || c == 65279))));
}

static inline bool anon_sym__character_set_2(int32_t c) {
  return (c < 8192
    ? (c < 160
      ? (c < ' '
        ? c == '\t'
        : c <= ' ')
      : (c <= 160 || c == 5760))
    : (c <= 8203 || (c < 12288
      ? (c < 8287
        ? c == 8239
        : c <= 8288)
      : (c <= 12288 || c == 65279))));
}

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(16);
      if (lookahead == 0) ADVANCE(75);
      if (lookahead == '"') ADVANCE(51);
      if (lookahead == '%') ADVANCE(27);
      if (lookahead == '\'') ADVANCE(55);
      if (lookahead == '(') ADVANCE(23);
      if (lookahead == ')') ADVANCE(24);
      if (lookahead == '*') ADVANCE(62);
      if (lookahead == ',') ADVANCE(60);
      if (lookahead == '.') ADVANCE(25);
      if (lookahead == '/') ADVANCE(13);
      if (lookahead == '=') ADVANCE(17);
      if (lookahead == '@') ADVANCE(35);
      if (lookahead == '[') ADVANCE(59);
      if (lookahead == ']') ADVANCE(61);
      if (lookahead == 'i') ADVANCE(29);
      if (lookahead == '{') ADVANCE(20);
      if (lookahead == '}') ADVANCE(22);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(50);
      if (anon_sym__character_set_1(lookahead)) SKIP(0)
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 1:
      if (lookahead == 0) ADVANCE(75);
      if (lookahead == '\n') ADVANCE(64);
      if (lookahead == '\r') ADVANCE(70);
      if (lookahead == '/') ADVANCE(13);
      if (anon_sym__character_set_2(lookahead)) SKIP(1)
      END_STATE();
    case 2:
      if (lookahead == 0) ADVANCE(76);
      if (lookahead == '\n') ADVANCE(63);
      if (lookahead == '\r') ADVANCE(69);
      if (lookahead == ' ') SKIP(2)
      if (lookahead == '%') ADVANCE(27);
      if (lookahead == '(') ADVANCE(23);
      if (lookahead == '.') ADVANCE(26);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(37);
      if (lookahead != 0 &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 3:
      if (lookahead == 0) ADVANCE(76);
      if (lookahead == '\n') ADVANCE(65);
      if (lookahead == '\r') ADVANCE(71);
      if (lookahead == ' ') SKIP(3)
      if (lookahead == '%') ADVANCE(27);
      if (lookahead == '(') ADVANCE(23);
      if (lookahead == '.') ADVANCE(26);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '=') ADVANCE(18);
      if (lookahead == '@') ADVANCE(36);
      if (lookahead == '{') ADVANCE(21);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(38);
      if (lookahead != 0 &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 4:
      if (lookahead == 0) ADVANCE(76);
      if (lookahead == '\n') ADVANCE(66);
      if (lookahead == '\r') ADVANCE(72);
      if (lookahead == ' ') SKIP(4)
      if (lookahead == '%') ADVANCE(27);
      if (lookahead == '(') ADVANCE(23);
      if (lookahead == '.') ADVANCE(26);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '{') ADVANCE(21);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(39);
      if (lookahead != 0 &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 5:
      if (lookahead == 0) ADVANCE(76);
      if (lookahead == '\n') ADVANCE(67);
      if (lookahead == '\r') ADVANCE(73);
      if (lookahead == ' ') SKIP(5)
      if (lookahead == '%') ADVANCE(27);
      if (lookahead == '(') ADVANCE(23);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(40);
      if (lookahead != 0 &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 6:
      if (lookahead == 0) ADVANCE(76);
      if (lookahead == '\n') ADVANCE(68);
      if (lookahead == '\r') ADVANCE(74);
      if (lookahead == ' ') SKIP(6)
      if (lookahead == '%') ADVANCE(27);
      if (lookahead == '(') ADVANCE(23);
      if (lookahead == '.') ADVANCE(26);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '@') ADVANCE(36);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(41);
      if (lookahead != 0 &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 7:
      if (lookahead == '"') ADVANCE(51);
      if (lookahead == '/') ADVANCE(53);
      if (anon_sym__character_set_1(lookahead)) ADVANCE(54);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 8:
      if (lookahead == '%') ADVANCE(27);
      if (lookahead == '(') ADVANCE(23);
      if (lookahead == ')') ADVANCE(24);
      if (lookahead == '.') ADVANCE(26);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '@') ADVANCE(36);
      if (lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(8)
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(42);
      if (lookahead != 0) ADVANCE(49);
      END_STATE();
    case 9:
      if (lookahead == '%') ADVANCE(27);
      if (lookahead == '(') ADVANCE(23);
      if (lookahead == ')') ADVANCE(24);
      if (lookahead == '.') ADVANCE(26);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(9)
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(43);
      if (lookahead != 0) ADVANCE(49);
      END_STATE();
    case 10:
      if (lookahead == '%') ADVANCE(27);
      if (lookahead == '(') ADVANCE(23);
      if (lookahead == ')') ADVANCE(24);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(10)
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(44);
      if (lookahead != 0) ADVANCE(49);
      END_STATE();
    case 11:
      if (lookahead == '%') ADVANCE(27);
      if (lookahead == '*') ADVANCE(62);
      if (lookahead == '/') ADVANCE(13);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(50);
      if (anon_sym__character_set_1(lookahead)) SKIP(11)
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 12:
      if (lookahead == '\'') ADVANCE(55);
      if (lookahead == '/') ADVANCE(57);
      if (anon_sym__character_set_1(lookahead)) ADVANCE(58);
      if (lookahead != 0) ADVANCE(56);
      END_STATE();
    case 13:
      if (lookahead == '*') ADVANCE(15);
      if (lookahead == '/') ADVANCE(78);
      END_STATE();
    case 14:
      if (lookahead == '*') ADVANCE(14);
      if (lookahead == '/') ADVANCE(77);
      if (lookahead != 0) ADVANCE(15);
      END_STATE();
    case 15:
      if (lookahead == '*') ADVANCE(14);
      if (lookahead != 0) ADVANCE(15);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_EQ);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '(' &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_import);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '(' &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(anon_sym_DOT);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '(' &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '(' &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym__identifier_without_ver);
      if (lookahead == 'm') ADVANCE(31);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym__identifier_without_ver);
      if (lookahead == 'o') ADVANCE(32);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(sym__identifier_without_ver);
      if (lookahead == 'p') ADVANCE(30);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(sym__identifier_without_ver);
      if (lookahead == 'r') ADVANCE(33);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(sym__identifier_without_ver);
      if (lookahead == 't') ADVANCE(19);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(sym__identifier_without_ver);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_AT);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '(' &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == 0) ADVANCE(76);
      if (lookahead == '\n') ADVANCE(63);
      if (lookahead == '\r') ADVANCE(69);
      if (lookahead == '%') ADVANCE(28);
      if (lookahead == '.') ADVANCE(26);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(37);
      if (lookahead != 0 &&
          lookahead != ' ' &&
          lookahead != '(' &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == 0) ADVANCE(76);
      if (lookahead == '\n') ADVANCE(65);
      if (lookahead == '\r') ADVANCE(71);
      if (lookahead == '%') ADVANCE(28);
      if (lookahead == '.') ADVANCE(26);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '=') ADVANCE(18);
      if (lookahead == '@') ADVANCE(36);
      if (lookahead == '{') ADVANCE(21);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(38);
      if (lookahead != 0 &&
          lookahead != ' ' &&
          lookahead != '(' &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == 0) ADVANCE(76);
      if (lookahead == '\n') ADVANCE(66);
      if (lookahead == '\r') ADVANCE(72);
      if (lookahead == '%') ADVANCE(28);
      if (lookahead == '.') ADVANCE(26);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '{') ADVANCE(21);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(39);
      if (lookahead != 0 &&
          lookahead != ' ' &&
          lookahead != '(' &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == 0) ADVANCE(76);
      if (lookahead == '\n') ADVANCE(67);
      if (lookahead == '\r') ADVANCE(73);
      if (lookahead == '%') ADVANCE(28);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(40);
      if (lookahead != 0 &&
          lookahead != ' ' &&
          lookahead != '(' &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == 0) ADVANCE(76);
      if (lookahead == '\n') ADVANCE(68);
      if (lookahead == '\r') ADVANCE(74);
      if (lookahead == '%') ADVANCE(28);
      if (lookahead == '.') ADVANCE(26);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '@') ADVANCE(36);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(41);
      if (lookahead != 0 &&
          lookahead != ' ' &&
          lookahead != '(' &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '%') ADVANCE(28);
      if (lookahead == '.') ADVANCE(26);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '@') ADVANCE(36);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(42);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '(' &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '%') ADVANCE(28);
      if (lookahead == '.') ADVANCE(26);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(43);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '(' &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '%') ADVANCE(28);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(44);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '(' &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '*') ADVANCE(47);
      if (lookahead == '/') ADVANCE(48);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '(' &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '*') ADVANCE(46);
      if (lookahead == '/') ADVANCE(49);
      if (lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == '(' ||
          lookahead == ')') ADVANCE(15);
      if (lookahead != 0) ADVANCE(47);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '*') ADVANCE(46);
      if (lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == '(' ||
          lookahead == ')') ADVANCE(15);
      if (lookahead != 0) ADVANCE(47);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == '(' ||
          lookahead == ')') ADVANCE(78);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(48);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(sym_text);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '(' &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(50);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(aux_sym_string_token1);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '*') ADVANCE(15);
      if (lookahead == '/') ADVANCE(78);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '/') ADVANCE(53);
      if (anon_sym__character_set_1(lookahead)) ADVANCE(54);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(52);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(aux_sym_string_token2);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead == '*') ADVANCE(15);
      if (lookahead == '/') ADVANCE(78);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead == '/') ADVANCE(57);
      if (anon_sym__character_set_1(lookahead)) ADVANCE(58);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(56);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(sym_wildcard);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(anon_sym_LF);
      if (lookahead == '\n') ADVANCE(63);
      if (lookahead == '\r') ADVANCE(69);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(37);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(anon_sym_LF);
      if (lookahead == '\n') ADVANCE(64);
      if (lookahead == '\r') ADVANCE(70);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(anon_sym_LF);
      if (lookahead == '\n') ADVANCE(65);
      if (lookahead == '\r') ADVANCE(71);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(38);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(anon_sym_LF);
      if (lookahead == '\n') ADVANCE(66);
      if (lookahead == '\r') ADVANCE(72);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(39);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(anon_sym_LF);
      if (lookahead == '\n') ADVANCE(67);
      if (lookahead == '\r') ADVANCE(73);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(40);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(anon_sym_LF);
      if (lookahead == '\n') ADVANCE(68);
      if (lookahead == '\r') ADVANCE(74);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(41);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(anon_sym_CR);
      if (lookahead == '\n') ADVANCE(63);
      if (lookahead == '\r') ADVANCE(69);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(37);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(anon_sym_CR);
      if (lookahead == '\n') ADVANCE(64);
      if (lookahead == '\r') ADVANCE(70);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(anon_sym_CR);
      if (lookahead == '\n') ADVANCE(65);
      if (lookahead == '\r') ADVANCE(71);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(38);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(anon_sym_CR);
      if (lookahead == '\n') ADVANCE(66);
      if (lookahead == '\r') ADVANCE(72);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(39);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(anon_sym_CR);
      if (lookahead == '\n') ADVANCE(67);
      if (lookahead == '\r') ADVANCE(73);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(40);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(anon_sym_CR);
      if (lookahead == '\n') ADVANCE(68);
      if (lookahead == '\r') ADVANCE(74);
      if (lookahead == '\t' ||
          lookahead == 160 ||
          lookahead == 5760 ||
          (8192 <= lookahead && lookahead <= 8203) ||
          lookahead == 8239 ||
          lookahead == 8287 ||
          lookahead == 8288 ||
          lookahead == 12288 ||
          lookahead == 65279) ADVANCE(41);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(anon_sym_);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(anon_sym_);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '(' &&
          lookahead != ')') ADVANCE(49);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(78);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 2},
  [7] = {.lex_state = 3},
  [8] = {.lex_state = 4},
  [9] = {.lex_state = 4},
  [10] = {.lex_state = 5},
  [11] = {.lex_state = 4},
  [12] = {.lex_state = 6},
  [13] = {.lex_state = 5},
  [14] = {.lex_state = 9},
  [15] = {.lex_state = 2},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 2},
  [18] = {.lex_state = 2},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 5},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 5},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 10},
  [28] = {.lex_state = 8},
  [29] = {.lex_state = 11},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 0},
  [33] = {.lex_state = 0},
  [34] = {.lex_state = 11},
  [35] = {.lex_state = 0},
  [36] = {.lex_state = 10},
  [37] = {.lex_state = 0},
  [38] = {.lex_state = 9},
  [39] = {.lex_state = 9},
  [40] = {.lex_state = 0},
  [41] = {.lex_state = 0},
  [42] = {.lex_state = 1},
  [43] = {.lex_state = 9},
  [44] = {.lex_state = 0},
  [45] = {.lex_state = 11},
  [46] = {.lex_state = 10},
  [47] = {.lex_state = 0},
  [48] = {.lex_state = 0},
  [49] = {.lex_state = 10},
  [50] = {.lex_state = 11},
  [51] = {.lex_state = 0},
  [52] = {.lex_state = 0},
  [53] = {.lex_state = 11},
  [54] = {.lex_state = 11},
  [55] = {.lex_state = 12},
  [56] = {.lex_state = 0},
  [57] = {.lex_state = 0},
  [58] = {.lex_state = 7},
  [59] = {.lex_state = 11},
  [60] = {.lex_state = 0},
  [61] = {.lex_state = 12},
  [62] = {.lex_state = 7},
  [63] = {.lex_state = 11},
  [64] = {.lex_state = 12},
  [65] = {.lex_state = 7},
  [66] = {.lex_state = 0},
  [67] = {.lex_state = 0},
  [68] = {.lex_state = 11},
  [69] = {.lex_state = 11},
  [70] = {.lex_state = 0},
  [71] = {.lex_state = 0},
  [72] = {.lex_state = 0},
  [73] = {.lex_state = 11},
  [74] = {.lex_state = 11},
  [75] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_import] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_PERCENT] = ACTIONS(1),
    [sym__identifier_without_ver] = ACTIONS(1),
    [anon_sym_AT] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [anon_sym_SQUOTE] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [sym_wildcard] = ACTIONS(1),
    [anon_sym_] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
  },
  [1] = {
    [sym_source_file] = STATE(70),
    [sym__body] = STATE(75),
    [sym__expr] = STATE(4),
    [sym_variable_def] = STATE(4),
    [sym_import_expr] = STATE(4),
    [sym_namespace] = STATE(4),
    [sym_command] = STATE(42),
    [sym__path] = STATE(6),
    [sym_identifier] = STATE(9),
    [sym_nested_identifier] = STATE(6),
    [sym__identifier_with_ver] = STATE(8),
    [anon_sym_import] = ACTIONS(5),
    [sym__identifier_without_ver] = ACTIONS(7),
    [sym_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(5), 1,
      anon_sym_import,
    ACTIONS(7), 1,
      sym__identifier_without_ver,
    STATE(8), 1,
      sym__identifier_with_ver,
    STATE(9), 1,
      sym_identifier,
    STATE(42), 1,
      sym_command,
    ACTIONS(9), 2,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
    STATE(6), 2,
      sym__path,
      sym_nested_identifier,
    STATE(3), 5,
      sym__expr,
      sym_variable_def,
      sym_import_expr,
      sym_namespace,
      aux_sym__body_repeat1,
  [34] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(13), 1,
      anon_sym_import,
    ACTIONS(16), 1,
      sym__identifier_without_ver,
    STATE(8), 1,
      sym__identifier_with_ver,
    STATE(9), 1,
      sym_identifier,
    STATE(42), 1,
      sym_command,
    ACTIONS(11), 2,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
    STATE(6), 2,
      sym__path,
      sym_nested_identifier,
    STATE(3), 5,
      sym__expr,
      sym_variable_def,
      sym_import_expr,
      sym_namespace,
      aux_sym__body_repeat1,
  [68] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(5), 1,
      anon_sym_import,
    ACTIONS(7), 1,
      sym__identifier_without_ver,
    STATE(8), 1,
      sym__identifier_with_ver,
    STATE(9), 1,
      sym_identifier,
    STATE(42), 1,
      sym_command,
    ACTIONS(19), 2,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
    STATE(6), 2,
      sym__path,
      sym_nested_identifier,
    STATE(2), 5,
      sym__expr,
      sym_variable_def,
      sym_import_expr,
      sym_namespace,
      aux_sym__body_repeat1,
  [102] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(5), 1,
      anon_sym_import,
    ACTIONS(7), 1,
      sym__identifier_without_ver,
    STATE(8), 1,
      sym__identifier_with_ver,
    STATE(9), 1,
      sym_identifier,
    STATE(42), 1,
      sym_command,
    STATE(71), 1,
      sym__body,
    STATE(6), 2,
      sym__path,
      sym_nested_identifier,
    STATE(4), 4,
      sym__expr,
      sym_variable_def,
      sym_import_expr,
      sym_namespace,
  [134] = 8,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_DOT,
    ACTIONS(25), 1,
      anon_sym_PERCENT,
    ACTIONS(27), 1,
      sym_text,
    ACTIONS(31), 1,
      sym_comment,
    STATE(54), 1,
      sym__dot,
    STATE(13), 2,
      sym_variable,
      aux_sym_command_repeat1,
    ACTIONS(29), 4,
      anon_sym_LF,
      anon_sym_CR,
      anon_sym_LF_CR,
      anon_sym_,
  [163] = 4,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(33), 1,
      anon_sym_EQ,
    ACTIONS(37), 1,
      anon_sym_AT,
    ACTIONS(35), 9,
      anon_sym_LBRACE,
      anon_sym_LPAREN,
      anon_sym_DOT,
      anon_sym_PERCENT,
      sym_text,
      anon_sym_LF,
      anon_sym_CR,
      anon_sym_LF_CR,
      anon_sym_,
  [184] = 2,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(35), 9,
      anon_sym_LBRACE,
      anon_sym_LPAREN,
      anon_sym_DOT,
      anon_sym_PERCENT,
      sym_text,
      anon_sym_LF,
      anon_sym_CR,
      anon_sym_LF_CR,
      anon_sym_,
  [199] = 3,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(39), 1,
      anon_sym_LBRACE,
    ACTIONS(41), 8,
      anon_sym_LPAREN,
      anon_sym_DOT,
      anon_sym_PERCENT,
      sym_text,
      anon_sym_LF,
      anon_sym_CR,
      anon_sym_LF_CR,
      anon_sym_,
  [216] = 6,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(43), 1,
      anon_sym_LPAREN,
    ACTIONS(46), 1,
      anon_sym_PERCENT,
    ACTIONS(49), 1,
      sym_text,
    STATE(10), 2,
      sym_variable,
      aux_sym_command_repeat1,
    ACTIONS(52), 4,
      anon_sym_LF,
      anon_sym_CR,
      anon_sym_LF_CR,
      anon_sym_,
  [239] = 2,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(54), 9,
      anon_sym_LBRACE,
      anon_sym_LPAREN,
      anon_sym_DOT,
      anon_sym_PERCENT,
      sym_text,
      anon_sym_LF,
      anon_sym_CR,
      anon_sym_LF_CR,
      anon_sym_,
  [254] = 3,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(56), 1,
      anon_sym_AT,
    ACTIONS(35), 8,
      anon_sym_LPAREN,
      anon_sym_DOT,
      anon_sym_PERCENT,
      sym_text,
      anon_sym_LF,
      anon_sym_CR,
      anon_sym_LF_CR,
      anon_sym_,
  [271] = 6,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(25), 1,
      anon_sym_PERCENT,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(58), 1,
      sym_text,
    STATE(10), 2,
      sym_variable,
      aux_sym_command_repeat1,
    ACTIONS(60), 4,
      anon_sym_LF,
      anon_sym_CR,
      anon_sym_LF_CR,
      anon_sym_,
  [294] = 8,
    ACTIONS(29), 1,
      anon_sym_RPAREN,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(62), 1,
      anon_sym_LPAREN,
    ACTIONS(64), 1,
      anon_sym_DOT,
    ACTIONS(66), 1,
      anon_sym_PERCENT,
    ACTIONS(68), 1,
      sym_text,
    STATE(59), 1,
      sym__dot,
    STATE(27), 2,
      sym_variable,
      aux_sym_command_repeat1,
  [320] = 2,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(70), 8,
      anon_sym_LPAREN,
      anon_sym_DOT,
      anon_sym_PERCENT,
      sym_text,
      anon_sym_LF,
      anon_sym_CR,
      anon_sym_LF_CR,
      anon_sym_,
  [334] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(72), 1,
      sym_number,
    ACTIONS(74), 1,
      anon_sym_DQUOTE,
    ACTIONS(76), 1,
      anon_sym_SQUOTE,
    ACTIONS(78), 1,
      anon_sym_LBRACK,
    ACTIONS(80), 1,
      anon_sym_RBRACK,
    STATE(66), 3,
      sym__primitive,
      sym_string,
      sym_array,
  [358] = 2,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(35), 8,
      anon_sym_LPAREN,
      anon_sym_DOT,
      anon_sym_PERCENT,
      sym_text,
      anon_sym_LF,
      anon_sym_CR,
      anon_sym_LF_CR,
      anon_sym_,
  [372] = 2,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(54), 8,
      anon_sym_LPAREN,
      anon_sym_DOT,
      anon_sym_PERCENT,
      sym_text,
      anon_sym_LF,
      anon_sym_CR,
      anon_sym_LF_CR,
      anon_sym_,
  [386] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(72), 1,
      sym_number,
    ACTIONS(74), 1,
      anon_sym_DQUOTE,
    ACTIONS(76), 1,
      anon_sym_SQUOTE,
    ACTIONS(78), 1,
      anon_sym_LBRACK,
    ACTIONS(82), 1,
      anon_sym_RBRACK,
    STATE(66), 3,
      sym__primitive,
      sym_string,
      sym_array,
  [410] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(74), 1,
      anon_sym_DQUOTE,
    ACTIONS(76), 1,
      anon_sym_SQUOTE,
    ACTIONS(78), 1,
      anon_sym_LBRACK,
    ACTIONS(84), 1,
      sym_number,
    ACTIONS(86), 1,
      anon_sym_RBRACK,
    STATE(60), 3,
      sym__primitive,
      sym_string,
      sym_array,
  [434] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(74), 1,
      anon_sym_DQUOTE,
    ACTIONS(76), 1,
      anon_sym_SQUOTE,
    ACTIONS(78), 1,
      anon_sym_LBRACK,
    ACTIONS(88), 1,
      sym_number,
    STATE(52), 3,
      sym__primitive,
      sym_string,
      sym_array,
  [455] = 2,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(90), 7,
      anon_sym_LPAREN,
      anon_sym_PERCENT,
      sym_text,
      anon_sym_LF,
      anon_sym_CR,
      anon_sym_LF_CR,
      anon_sym_,
  [468] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(72), 1,
      sym_number,
    ACTIONS(74), 1,
      anon_sym_DQUOTE,
    ACTIONS(76), 1,
      anon_sym_SQUOTE,
    ACTIONS(78), 1,
      anon_sym_LBRACK,
    STATE(66), 3,
      sym__primitive,
      sym_string,
      sym_array,
  [489] = 2,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(92), 7,
      anon_sym_LPAREN,
      anon_sym_PERCENT,
      sym_text,
      anon_sym_LF,
      anon_sym_CR,
      anon_sym_LF_CR,
      anon_sym_,
  [502] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(96), 1,
      anon_sym_AT,
    ACTIONS(35), 2,
      anon_sym_import,
      sym__identifier_without_ver,
    ACTIONS(94), 3,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_DOT,
  [518] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(100), 2,
      anon_sym_import,
      sym__identifier_without_ver,
    ACTIONS(98), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
  [532] = 6,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(60), 1,
      anon_sym_RPAREN,
    ACTIONS(62), 1,
      anon_sym_LPAREN,
    ACTIONS(66), 1,
      anon_sym_PERCENT,
    ACTIONS(102), 1,
      sym_text,
    STATE(36), 2,
      sym_variable,
      aux_sym_command_repeat1,
  [552] = 3,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(104), 1,
      anon_sym_AT,
    ACTIONS(35), 5,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_DOT,
      anon_sym_PERCENT,
      sym_text,
  [566] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(106), 1,
      sym__identifier_without_ver,
    STATE(43), 1,
      sym__identifier_with_ver,
    STATE(72), 1,
      sym_command,
    STATE(14), 3,
      sym__path,
      sym_identifier,
      sym_nested_identifier,
  [584] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(110), 2,
      anon_sym_import,
      sym__identifier_without_ver,
    ACTIONS(108), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
  [598] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(114), 2,
      anon_sym_import,
      sym__identifier_without_ver,
    ACTIONS(112), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
  [612] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(120), 1,
      anon_sym_DOT,
    STATE(50), 1,
      sym__dot,
    ACTIONS(116), 2,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
    ACTIONS(118), 2,
      anon_sym_import,
      sym__identifier_without_ver,
  [630] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(124), 2,
      anon_sym_import,
      sym__identifier_without_ver,
    ACTIONS(122), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
  [644] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(106), 1,
      sym__identifier_without_ver,
    STATE(43), 1,
      sym__identifier_with_ver,
    STATE(67), 1,
      sym_command,
    STATE(14), 3,
      sym__path,
      sym_identifier,
      sym_nested_identifier,
  [662] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(128), 2,
      anon_sym_import,
      sym__identifier_without_ver,
    ACTIONS(126), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
  [676] = 6,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(52), 1,
      anon_sym_RPAREN,
    ACTIONS(130), 1,
      anon_sym_LPAREN,
    ACTIONS(133), 1,
      anon_sym_PERCENT,
    ACTIONS(136), 1,
      sym_text,
    STATE(36), 2,
      sym_variable,
      aux_sym_command_repeat1,
  [696] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(141), 2,
      anon_sym_import,
      sym__identifier_without_ver,
    ACTIONS(139), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
  [710] = 2,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(54), 5,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_DOT,
      anon_sym_PERCENT,
      sym_text,
  [721] = 2,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(70), 5,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_DOT,
      anon_sym_PERCENT,
      sym_text,
  [732] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(70), 2,
      anon_sym_import,
      sym__identifier_without_ver,
    ACTIONS(143), 3,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_DOT,
  [745] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(54), 2,
      anon_sym_import,
      sym__identifier_without_ver,
    ACTIONS(145), 3,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_DOT,
  [758] = 3,
    ACTIONS(31), 1,
      sym_comment,
    STATE(51), 1,
      sym__newline,
    ACTIONS(147), 4,
      anon_sym_LF,
      anon_sym_CR,
      anon_sym_LF_CR,
      anon_sym_,
  [771] = 2,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(35), 5,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_DOT,
      anon_sym_PERCENT,
      sym_text,
  [782] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(35), 2,
      anon_sym_import,
      sym__identifier_without_ver,
    ACTIONS(94), 3,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_DOT,
  [795] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(149), 1,
      sym__identifier_without_ver,
    STATE(44), 1,
      sym__identifier_with_ver,
    STATE(32), 3,
      sym__path,
      sym_identifier,
      sym_nested_identifier,
  [810] = 2,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(90), 4,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_PERCENT,
      sym_text,
  [820] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(151), 2,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
    ACTIONS(153), 2,
      anon_sym_import,
      sym__identifier_without_ver,
  [832] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 2,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
    ACTIONS(157), 2,
      anon_sym_import,
      sym__identifier_without_ver,
  [844] = 2,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(92), 4,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_PERCENT,
      sym_text,
  [854] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(149), 1,
      sym__identifier_without_ver,
    ACTIONS(159), 1,
      sym_wildcard,
    STATE(40), 1,
      sym_identifier,
    STATE(44), 1,
      sym__identifier_with_ver,
  [870] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(161), 2,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
    ACTIONS(163), 2,
      anon_sym_import,
      sym__identifier_without_ver,
  [882] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(165), 2,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
    ACTIONS(167), 2,
      anon_sym_import,
      sym__identifier_without_ver,
  [894] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(169), 3,
      anon_sym_PERCENT,
      sym__identifier_without_ver,
      sym_number,
  [903] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(171), 1,
      sym__identifier_without_ver,
    STATE(15), 1,
      sym_identifier,
    STATE(17), 1,
      sym__identifier_with_ver,
  [916] = 4,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(173), 1,
      anon_sym_SQUOTE,
    ACTIONS(175), 1,
      aux_sym_string_token2,
    STATE(55), 1,
      aux_sym_string_repeat2,
  [929] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(178), 1,
      anon_sym_COMMA,
    ACTIONS(181), 1,
      anon_sym_RBRACK,
    STATE(56), 1,
      aux_sym_array_repeat1,
  [942] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(80), 1,
      anon_sym_RBRACK,
    ACTIONS(183), 1,
      anon_sym_COMMA,
    STATE(56), 1,
      aux_sym_array_repeat1,
  [955] = 4,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(185), 1,
      anon_sym_DQUOTE,
    ACTIONS(187), 1,
      aux_sym_string_token1,
    STATE(58), 1,
      aux_sym_string_repeat1,
  [968] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(106), 1,
      sym__identifier_without_ver,
    STATE(39), 1,
      sym_identifier,
    STATE(43), 1,
      sym__identifier_with_ver,
  [981] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(190), 1,
      anon_sym_COMMA,
    ACTIONS(192), 1,
      anon_sym_RBRACK,
    STATE(57), 1,
      aux_sym_array_repeat1,
  [994] = 4,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(194), 1,
      anon_sym_SQUOTE,
    ACTIONS(196), 1,
      aux_sym_string_token2,
    STATE(55), 1,
      aux_sym_string_repeat2,
  [1007] = 4,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(194), 1,
      anon_sym_DQUOTE,
    ACTIONS(198), 1,
      aux_sym_string_token1,
    STATE(58), 1,
      aux_sym_string_repeat1,
  [1020] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(200), 3,
      anon_sym_PERCENT,
      sym__identifier_without_ver,
      sym_number,
  [1029] = 4,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(202), 1,
      anon_sym_SQUOTE,
    ACTIONS(204), 1,
      aux_sym_string_token2,
    STATE(61), 1,
      aux_sym_string_repeat2,
  [1042] = 4,
    ACTIONS(31), 1,
      sym_comment,
    ACTIONS(202), 1,
      anon_sym_DQUOTE,
    ACTIONS(206), 1,
      aux_sym_string_token1,
    STATE(62), 1,
      aux_sym_string_repeat1,
  [1055] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(181), 2,
      anon_sym_COMMA,
      anon_sym_RBRACK,
  [1063] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(208), 1,
      anon_sym_RPAREN,
  [1070] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(210), 1,
      sym__identifier_without_ver,
  [1077] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(212), 1,
      sym__identifier_without_ver,
  [1084] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(214), 1,
      ts_builtin_sym_end,
  [1091] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(216), 1,
      anon_sym_RBRACE,
  [1098] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(218), 1,
      anon_sym_RPAREN,
  [1105] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(220), 1,
      sym__identifier_without_ver,
  [1112] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(222), 1,
      sym__identifier_without_ver,
  [1119] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(224), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 34,
  [SMALL_STATE(4)] = 68,
  [SMALL_STATE(5)] = 102,
  [SMALL_STATE(6)] = 134,
  [SMALL_STATE(7)] = 163,
  [SMALL_STATE(8)] = 184,
  [SMALL_STATE(9)] = 199,
  [SMALL_STATE(10)] = 216,
  [SMALL_STATE(11)] = 239,
  [SMALL_STATE(12)] = 254,
  [SMALL_STATE(13)] = 271,
  [SMALL_STATE(14)] = 294,
  [SMALL_STATE(15)] = 320,
  [SMALL_STATE(16)] = 334,
  [SMALL_STATE(17)] = 358,
  [SMALL_STATE(18)] = 372,
  [SMALL_STATE(19)] = 386,
  [SMALL_STATE(20)] = 410,
  [SMALL_STATE(21)] = 434,
  [SMALL_STATE(22)] = 455,
  [SMALL_STATE(23)] = 468,
  [SMALL_STATE(24)] = 489,
  [SMALL_STATE(25)] = 502,
  [SMALL_STATE(26)] = 518,
  [SMALL_STATE(27)] = 532,
  [SMALL_STATE(28)] = 552,
  [SMALL_STATE(29)] = 566,
  [SMALL_STATE(30)] = 584,
  [SMALL_STATE(31)] = 598,
  [SMALL_STATE(32)] = 612,
  [SMALL_STATE(33)] = 630,
  [SMALL_STATE(34)] = 644,
  [SMALL_STATE(35)] = 662,
  [SMALL_STATE(36)] = 676,
  [SMALL_STATE(37)] = 696,
  [SMALL_STATE(38)] = 710,
  [SMALL_STATE(39)] = 721,
  [SMALL_STATE(40)] = 732,
  [SMALL_STATE(41)] = 745,
  [SMALL_STATE(42)] = 758,
  [SMALL_STATE(43)] = 771,
  [SMALL_STATE(44)] = 782,
  [SMALL_STATE(45)] = 795,
  [SMALL_STATE(46)] = 810,
  [SMALL_STATE(47)] = 820,
  [SMALL_STATE(48)] = 832,
  [SMALL_STATE(49)] = 844,
  [SMALL_STATE(50)] = 854,
  [SMALL_STATE(51)] = 870,
  [SMALL_STATE(52)] = 882,
  [SMALL_STATE(53)] = 894,
  [SMALL_STATE(54)] = 903,
  [SMALL_STATE(55)] = 916,
  [SMALL_STATE(56)] = 929,
  [SMALL_STATE(57)] = 942,
  [SMALL_STATE(58)] = 955,
  [SMALL_STATE(59)] = 968,
  [SMALL_STATE(60)] = 981,
  [SMALL_STATE(61)] = 994,
  [SMALL_STATE(62)] = 1007,
  [SMALL_STATE(63)] = 1020,
  [SMALL_STATE(64)] = 1029,
  [SMALL_STATE(65)] = 1042,
  [SMALL_STATE(66)] = 1055,
  [SMALL_STATE(67)] = 1063,
  [SMALL_STATE(68)] = 1070,
  [SMALL_STATE(69)] = 1077,
  [SMALL_STATE(70)] = 1084,
  [SMALL_STATE(71)] = 1091,
  [SMALL_STATE(72)] = 1098,
  [SMALL_STATE(73)] = 1105,
  [SMALL_STATE(74)] = 1112,
  [SMALL_STATE(75)] = 1119,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(45),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(7),
  [9] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__body, 2),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__body_repeat1, 2),
  [13] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__body_repeat1, 2), SHIFT_REPEAT(45),
  [16] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__body_repeat1, 2), SHIFT_REPEAT(7),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__body, 1),
  [21] = {.entry = {.count = 1, .reusable = false}}, SHIFT(34),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(54),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(53),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(13),
  [29] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_command, 1, .production_id = 1),
  [31] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(21),
  [35] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_identifier, 1),
  [37] = {.entry = {.count = 1, .reusable = false}}, SHIFT(68),
  [39] = {.entry = {.count = 1, .reusable = false}}, SHIFT(5),
  [41] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__path, 1),
  [43] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_command_repeat1, 2), SHIFT_REPEAT(34),
  [46] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_command_repeat1, 2), SHIFT_REPEAT(53),
  [49] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_command_repeat1, 2), SHIFT_REPEAT(10),
  [52] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_command_repeat1, 2),
  [54] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__identifier_with_ver, 3),
  [56] = {.entry = {.count = 1, .reusable = false}}, SHIFT(74),
  [58] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [60] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_command, 2, .production_id = 2),
  [62] = {.entry = {.count = 1, .reusable = false}}, SHIFT(29),
  [64] = {.entry = {.count = 1, .reusable = false}}, SHIFT(59),
  [66] = {.entry = {.count = 1, .reusable = false}}, SHIFT(63),
  [68] = {.entry = {.count = 1, .reusable = false}}, SHIFT(27),
  [70] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_nested_identifier, 3, .production_id = 4),
  [72] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [74] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [76] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [78] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [80] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [82] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [84] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [86] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [88] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [90] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_variable, 2, .production_id = 3),
  [92] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_command_repeat1, 3),
  [94] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_identifier, 1),
  [96] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [98] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array, 2),
  [100] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array, 2),
  [102] = {.entry = {.count = 1, .reusable = false}}, SHIFT(36),
  [104] = {.entry = {.count = 1, .reusable = false}}, SHIFT(73),
  [106] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [108] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array, 5),
  [110] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array, 5),
  [112] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array, 4),
  [114] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array, 4),
  [116] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_expr, 2),
  [118] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_import_expr, 2),
  [120] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [122] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array, 3),
  [124] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array, 3),
  [126] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 2),
  [128] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 2),
  [130] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_command_repeat1, 2), SHIFT_REPEAT(29),
  [133] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_command_repeat1, 2), SHIFT_REPEAT(63),
  [136] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_command_repeat1, 2), SHIFT_REPEAT(36),
  [139] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3),
  [141] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 3),
  [143] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_nested_identifier, 3, .production_id = 4),
  [145] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__identifier_with_ver, 3),
  [147] = {.entry = {.count = 1, .reusable = false}}, SHIFT(51),
  [149] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [151] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_namespace, 4, .production_id = 5),
  [153] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_namespace, 4, .production_id = 5),
  [155] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_expr, 4),
  [157] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_import_expr, 4),
  [159] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [161] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr, 2),
  [163] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expr, 2),
  [165] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable_def, 3),
  [167] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_variable_def, 3),
  [169] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [171] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [173] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_repeat2, 2),
  [175] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_string_repeat2, 2), SHIFT_REPEAT(55),
  [178] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_array_repeat1, 2), SHIFT_REPEAT(23),
  [181] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_array_repeat1, 2),
  [183] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [185] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_repeat1, 2),
  [187] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_string_repeat1, 2), SHIFT_REPEAT(58),
  [190] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [192] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [194] = {.entry = {.count = 1, .reusable = false}}, SHIFT(37),
  [196] = {.entry = {.count = 1, .reusable = false}}, SHIFT(55),
  [198] = {.entry = {.count = 1, .reusable = false}}, SHIFT(58),
  [200] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [202] = {.entry = {.count = 1, .reusable = false}}, SHIFT(35),
  [204] = {.entry = {.count = 1, .reusable = false}}, SHIFT(61),
  [206] = {.entry = {.count = 1, .reusable = false}}, SHIFT(62),
  [208] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [210] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [212] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [214] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [216] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [218] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [220] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [222] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [224] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_repkg(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
