#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 48
#define LARGE_STATE_COUNT 3
#define SYMBOL_COUNT 29
#define ALIAS_COUNT 0
#define TOKEN_COUNT 16
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 8
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 12

enum {
  sym_identifier = 1,
  anon_sym_LBRACE = 2,
  anon_sym_RBRACE = 3,
  anon_sym_COLON = 4,
  anon_sym_LPAREN = 5,
  anon_sym_RPAREN = 6,
  anon_sym_DOLLAR = 7,
  sym_number = 8,
  anon_sym_DQUOTE = 9,
  anon_sym_SQUOTE = 10,
  sym_unescaped_double_string_fragment = 11,
  sym_unescaped_single_string_fragment = 12,
  sym_text = 13,
  anon_sym_DOT = 14,
  sym_comment = 15,
  sym_source_file = 16,
  sym_object = 17,
  sym_definition = 18,
  sym_func_def = 19,
  sym__expr = 20,
  sym_command = 21,
  sym_string = 22,
  sym_nested_identifier = 23,
  aux_sym_object_repeat1 = 24,
  aux_sym_func_def_repeat1 = 25,
  aux_sym_command_repeat1 = 26,
  aux_sym_string_repeat1 = 27,
  aux_sym_string_repeat2 = 28,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_COLON] = ":",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_DOLLAR] = "$",
  [sym_number] = "number",
  [anon_sym_DQUOTE] = "\"",
  [anon_sym_SQUOTE] = "'",
  [sym_unescaped_double_string_fragment] = "string_fragment",
  [sym_unescaped_single_string_fragment] = "string_fragment",
  [sym_text] = "text",
  [anon_sym_DOT] = ".",
  [sym_comment] = "comment",
  [sym_source_file] = "source_file",
  [sym_object] = "object",
  [sym_definition] = "definition",
  [sym_func_def] = "func_def",
  [sym__expr] = "_expr",
  [sym_command] = "command",
  [sym_string] = "string",
  [sym_nested_identifier] = "nested_identifier",
  [aux_sym_object_repeat1] = "object_repeat1",
  [aux_sym_func_def_repeat1] = "func_def_repeat1",
  [aux_sym_command_repeat1] = "command_repeat1",
  [aux_sym_string_repeat1] = "string_repeat1",
  [aux_sym_string_repeat2] = "string_repeat2",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_identifier] = sym_identifier,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_DOLLAR] = anon_sym_DOLLAR,
  [sym_number] = sym_number,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [anon_sym_SQUOTE] = anon_sym_SQUOTE,
  [sym_unescaped_double_string_fragment] = sym_unescaped_double_string_fragment,
  [sym_unescaped_single_string_fragment] = sym_unescaped_double_string_fragment,
  [sym_text] = sym_text,
  [anon_sym_DOT] = anon_sym_DOT,
  [sym_comment] = sym_comment,
  [sym_source_file] = sym_source_file,
  [sym_object] = sym_object,
  [sym_definition] = sym_definition,
  [sym_func_def] = sym_func_def,
  [sym__expr] = sym__expr,
  [sym_command] = sym_command,
  [sym_string] = sym_string,
  [sym_nested_identifier] = sym_nested_identifier,
  [aux_sym_object_repeat1] = aux_sym_object_repeat1,
  [aux_sym_func_def_repeat1] = aux_sym_func_def_repeat1,
  [aux_sym_command_repeat1] = aux_sym_command_repeat1,
  [aux_sym_string_repeat1] = aux_sym_string_repeat1,
  [aux_sym_string_repeat2] = aux_sym_string_repeat2,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
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
  [anon_sym_DOLLAR] = {
    .visible = true,
    .named = false,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SQUOTE] = {
    .visible = true,
    .named = false,
  },
  [sym_unescaped_double_string_fragment] = {
    .visible = true,
    .named = true,
  },
  [sym_unescaped_single_string_fragment] = {
    .visible = true,
    .named = true,
  },
  [sym_text] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_DOT] = {
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
  [sym_object] = {
    .visible = true,
    .named = true,
  },
  [sym_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_func_def] = {
    .visible = true,
    .named = true,
  },
  [sym__expr] = {
    .visible = false,
    .named = true,
  },
  [sym_command] = {
    .visible = true,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_nested_identifier] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_object_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_func_def_repeat1] = {
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
};

enum {
  field_args = 1,
  field_child = 2,
  field_commands = 3,
  field_expr = 4,
  field_id = 5,
  field_name = 6,
  field_path = 7,
  field_return = 8,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_args] = "args",
  [field_child] = "child",
  [field_commands] = "commands",
  [field_expr] = "expr",
  [field_id] = "id",
  [field_name] = "name",
  [field_path] = "path",
  [field_return] = "return",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 2},
  [3] = {.index = 3, .length = 1},
  [4] = {.index = 4, .length = 1},
  [5] = {.index = 5, .length = 1},
  [6] = {.index = 6, .length = 2},
  [7] = {.index = 8, .length = 2},
  [8] = {.index = 10, .length = 1},
  [9] = {.index = 11, .length = 1},
  [10] = {.index = 12, .length = 2},
  [11] = {.index = 14, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_child, 1},
  [1] =
    {field_expr, 2},
    {field_id, 0},
  [3] =
    {field_path, 1},
  [4] =
    {field_return, 1},
  [5] =
    {field_commands, 1},
  [6] =
    {field_commands, 1},
    {field_return, 2},
  [8] =
    {field_name, 2},
    {field_path, 0},
  [10] =
    {field_return, 3},
  [11] =
    {field_commands, 3},
  [12] =
    {field_args, 3},
    {field_path, 1},
  [14] =
    {field_commands, 3},
    {field_return, 4},
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
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 47,
};

static inline bool anon_sym_DQUOTE_character_set_1(int32_t c) {
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

static inline bool sym_text_character_set_1(int32_t c) {
  return (c < 8192
    ? (c < 160
      ? (c < '\r'
        ? (c >= '\t' && c <= '\n')
        : c <= '\r')
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
      if (eof) ADVANCE(7);
      if (lookahead == '"') ADVANCE(15);
      if (lookahead == '$') ADVANCE(13);
      if (lookahead == '\'') ADVANCE(16);
      if (lookahead == '(') ADVANCE(11);
      if (lookahead == ')') ADVANCE(12);
      if (lookahead == '.') ADVANCE(36);
      if (lookahead == '/') ADVANCE(4);
      if (lookahead == ':') ADVANCE(10);
      if (lookahead == '{') ADVANCE(8);
      if (lookahead == '}') ADVANCE(9);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(14);
      if (anon_sym_DQUOTE_character_set_1(lookahead)) SKIP(0)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(35);
      END_STATE();
    case 1:
      if (lookahead == ' ') SKIP(1)
      if (lookahead == ')') ADVANCE(12);
      if (lookahead == '/') ADVANCE(30);
      if (sym_text_character_set_1(lookahead)) ADVANCE(33);
      if (lookahead != 0) ADVANCE(34);
      END_STATE();
    case 2:
      if (lookahead == '"') ADVANCE(15);
      if (lookahead == '/') ADVANCE(18);
      if (anon_sym_DQUOTE_character_set_1(lookahead)) ADVANCE(21);
      if (lookahead != 0 &&
          lookahead != '\\') ADVANCE(22);
      END_STATE();
    case 3:
      if (lookahead == '\'') ADVANCE(16);
      if (lookahead == '/') ADVANCE(24);
      if (anon_sym_DQUOTE_character_set_1(lookahead)) ADVANCE(27);
      if (lookahead != 0 &&
          lookahead != '\\') ADVANCE(28);
      END_STATE();
    case 4:
      if (lookahead == '*') ADVANCE(6);
      if (lookahead == '/') ADVANCE(38);
      END_STATE();
    case 5:
      if (lookahead == '*') ADVANCE(5);
      if (lookahead == '/') ADVANCE(37);
      if (lookahead != 0) ADVANCE(6);
      END_STATE();
    case 6:
      if (lookahead == '*') ADVANCE(5);
      if (lookahead != 0) ADVANCE(6);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_DOLLAR);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(14);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(sym_unescaped_double_string_fragment);
      if (lookahead == '\n') ADVANCE(22);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(17);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(sym_unescaped_double_string_fragment);
      if (lookahead == '*') ADVANCE(20);
      if (lookahead == '/') ADVANCE(17);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(22);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(sym_unescaped_double_string_fragment);
      if (lookahead == '*') ADVANCE(19);
      if (lookahead == '/') ADVANCE(22);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(20);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(sym_unescaped_double_string_fragment);
      if (lookahead == '*') ADVANCE(19);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(20);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(sym_unescaped_double_string_fragment);
      if (lookahead == '/') ADVANCE(18);
      if (anon_sym_DQUOTE_character_set_1(lookahead)) ADVANCE(21);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(22);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(sym_unescaped_double_string_fragment);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(22);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(sym_unescaped_single_string_fragment);
      if (lookahead == '\n') ADVANCE(28);
      if (lookahead != 0 &&
          lookahead != '\'' &&
          lookahead != '\\') ADVANCE(23);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(sym_unescaped_single_string_fragment);
      if (lookahead == '*') ADVANCE(26);
      if (lookahead == '/') ADVANCE(23);
      if (lookahead != 0 &&
          lookahead != '\'' &&
          lookahead != '\\') ADVANCE(28);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(sym_unescaped_single_string_fragment);
      if (lookahead == '*') ADVANCE(25);
      if (lookahead == '/') ADVANCE(28);
      if (lookahead != 0 &&
          lookahead != '\'' &&
          lookahead != '\\') ADVANCE(26);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(sym_unescaped_single_string_fragment);
      if (lookahead == '*') ADVANCE(25);
      if (lookahead != 0 &&
          lookahead != '\'' &&
          lookahead != '\\') ADVANCE(26);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(sym_unescaped_single_string_fragment);
      if (lookahead == '/') ADVANCE(24);
      if (anon_sym_DQUOTE_character_set_1(lookahead)) ADVANCE(27);
      if (lookahead != 0 &&
          lookahead != '\'' &&
          lookahead != '\\') ADVANCE(28);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(sym_unescaped_single_string_fragment);
      if (lookahead != 0 &&
          lookahead != '\'' &&
          lookahead != '\\') ADVANCE(28);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '\n') ADVANCE(34);
      if (lookahead == ' ' ||
          lookahead == ')') ADVANCE(38);
      if (lookahead != 0) ADVANCE(29);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '*') ADVANCE(32);
      if (lookahead == '/') ADVANCE(29);
      if (lookahead != 0 &&
          lookahead != ' ' &&
          lookahead != ')') ADVANCE(34);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '*') ADVANCE(31);
      if (lookahead == '/') ADVANCE(34);
      if (lookahead == ' ' ||
          lookahead == ')') ADVANCE(6);
      if (lookahead != 0) ADVANCE(32);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '*') ADVANCE(31);
      if (lookahead == ' ' ||
          lookahead == ')') ADVANCE(6);
      if (lookahead != 0) ADVANCE(32);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '/') ADVANCE(30);
      if (sym_text_character_set_1(lookahead)) ADVANCE(33);
      if (lookahead != 0 &&
          lookahead != ' ' &&
          lookahead != ')') ADVANCE(34);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(sym_text);
      if (lookahead != 0 &&
          lookahead != ' ' &&
          lookahead != ')') ADVANCE(34);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(sym_identifier);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(35);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(38);
      END_STATE();
    default:
      return false;
  }
}

static bool ts_lex_keywords(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      ACCEPT_TOKEN(ts_builtin_sym_end);
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
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 1},
  [16] = {.lex_state = 3},
  [17] = {.lex_state = 1},
  [18] = {.lex_state = 3},
  [19] = {.lex_state = 1},
  [20] = {.lex_state = 2},
  [21] = {.lex_state = 2},
  [22] = {.lex_state = 2},
  [23] = {.lex_state = 3},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 0},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 0},
  [33] = {.lex_state = 0},
  [34] = {.lex_state = 0},
  [35] = {.lex_state = 0},
  [36] = {.lex_state = 0},
  [37] = {.lex_state = 0},
  [38] = {.lex_state = 0},
  [39] = {.lex_state = 0},
  [40] = {.lex_state = 0},
  [41] = {.lex_state = 0},
  [42] = {.lex_state = 0},
  [43] = {.lex_state = 0},
  [44] = {.lex_state = 0},
  [45] = {.lex_state = 0},
  [46] = {.lex_state = 0},
  [47] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_DOLLAR] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [anon_sym_SQUOTE] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
  },
  [1] = {
    [sym_source_file] = STATE(45),
    [sym_object] = STATE(46),
    [anon_sym_LBRACE] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
  },
  [2] = {
    [sym_object] = STATE(42),
    [sym_definition] = STATE(13),
    [sym__expr] = STATE(42),
    [sym_command] = STATE(4),
    [sym_string] = STATE(42),
    [aux_sym_object_repeat1] = STATE(13),
    [aux_sym_func_def_repeat1] = STATE(4),
    [sym_identifier] = ACTIONS(7),
    [anon_sym_LBRACE] = ACTIONS(5),
    [anon_sym_RBRACE] = ACTIONS(9),
    [anon_sym_DOLLAR] = ACTIONS(11),
    [sym_number] = ACTIONS(13),
    [anon_sym_DQUOTE] = ACTIONS(15),
    [anon_sym_SQUOTE] = ACTIONS(17),
    [sym_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(5), 1,
      anon_sym_LBRACE,
    ACTIONS(11), 1,
      anon_sym_DOLLAR,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(19), 1,
      anon_sym_RBRACE,
    ACTIONS(21), 1,
      sym_number,
    STATE(5), 2,
      sym_command,
      aux_sym_func_def_repeat1,
    STATE(38), 3,
      sym_object,
      sym__expr,
      sym_string,
  [31] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(5), 1,
      anon_sym_LBRACE,
    ACTIONS(11), 1,
      anon_sym_DOLLAR,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(23), 1,
      anon_sym_RBRACE,
    ACTIONS(25), 1,
      sym_number,
    STATE(9), 2,
      sym_command,
      aux_sym_func_def_repeat1,
    STATE(47), 3,
      sym_object,
      sym__expr,
      sym_string,
  [62] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(5), 1,
      anon_sym_LBRACE,
    ACTIONS(11), 1,
      anon_sym_DOLLAR,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_RBRACE,
    ACTIONS(29), 1,
      sym_number,
    STATE(9), 2,
      sym_command,
      aux_sym_func_def_repeat1,
    STATE(44), 3,
      sym_object,
      sym__expr,
      sym_string,
  [93] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_LBRACE,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      sym_number,
    STATE(34), 4,
      sym_object,
      sym_func_def,
      sym__expr,
      sym_string,
  [118] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_DOT,
    ACTIONS(37), 6,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOLLAR,
      sym_number,
      anon_sym_DQUOTE,
      anon_sym_SQUOTE,
  [136] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(43), 8,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      anon_sym_DOLLAR,
      sym_number,
      anon_sym_DQUOTE,
      anon_sym_SQUOTE,
      anon_sym_DOT,
  [150] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(47), 1,
      anon_sym_DOLLAR,
    STATE(9), 2,
      sym_command,
      aux_sym_func_def_repeat1,
    ACTIONS(45), 5,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      sym_number,
      anon_sym_DQUOTE,
      anon_sym_SQUOTE,
  [168] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(50), 6,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOLLAR,
      sym_number,
      anon_sym_DQUOTE,
      anon_sym_SQUOTE,
  [180] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(52), 6,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOLLAR,
      sym_number,
      anon_sym_DQUOTE,
      anon_sym_SQUOTE,
  [192] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(54), 1,
      anon_sym_RBRACE,
    STATE(13), 2,
      sym_definition,
      aux_sym_object_repeat1,
  [206] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(56), 1,
      anon_sym_RBRACE,
    STATE(14), 2,
      sym_definition,
      aux_sym_object_repeat1,
  [220] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(58), 1,
      sym_identifier,
    ACTIONS(61), 1,
      anon_sym_RBRACE,
    STATE(14), 2,
      sym_definition,
      aux_sym_object_repeat1,
  [234] = 4,
    ACTIONS(63), 1,
      anon_sym_RPAREN,
    ACTIONS(65), 1,
      sym_text,
    ACTIONS(67), 1,
      sym_comment,
    STATE(19), 1,
      aux_sym_command_repeat1,
  [247] = 4,
    ACTIONS(67), 1,
      sym_comment,
    ACTIONS(69), 1,
      anon_sym_SQUOTE,
    ACTIONS(71), 1,
      sym_unescaped_single_string_fragment,
    STATE(23), 1,
      aux_sym_string_repeat2,
  [260] = 4,
    ACTIONS(67), 1,
      sym_comment,
    ACTIONS(73), 1,
      anon_sym_RPAREN,
    ACTIONS(75), 1,
      sym_text,
    STATE(15), 1,
      aux_sym_command_repeat1,
  [273] = 4,
    ACTIONS(67), 1,
      sym_comment,
    ACTIONS(77), 1,
      anon_sym_SQUOTE,
    ACTIONS(79), 1,
      sym_unescaped_single_string_fragment,
    STATE(18), 1,
      aux_sym_string_repeat2,
  [286] = 4,
    ACTIONS(67), 1,
      sym_comment,
    ACTIONS(82), 1,
      anon_sym_RPAREN,
    ACTIONS(84), 1,
      sym_text,
    STATE(19), 1,
      aux_sym_command_repeat1,
  [299] = 4,
    ACTIONS(67), 1,
      sym_comment,
    ACTIONS(87), 1,
      anon_sym_DQUOTE,
    ACTIONS(89), 1,
      sym_unescaped_double_string_fragment,
    STATE(20), 1,
      aux_sym_string_repeat1,
  [312] = 4,
    ACTIONS(67), 1,
      sym_comment,
    ACTIONS(69), 1,
      anon_sym_DQUOTE,
    ACTIONS(92), 1,
      sym_unescaped_double_string_fragment,
    STATE(22), 1,
      aux_sym_string_repeat1,
  [325] = 4,
    ACTIONS(67), 1,
      sym_comment,
    ACTIONS(94), 1,
      anon_sym_DQUOTE,
    ACTIONS(96), 1,
      sym_unescaped_double_string_fragment,
    STATE(20), 1,
      aux_sym_string_repeat1,
  [338] = 4,
    ACTIONS(67), 1,
      sym_comment,
    ACTIONS(94), 1,
      anon_sym_SQUOTE,
    ACTIONS(98), 1,
      sym_unescaped_single_string_fragment,
    STATE(18), 1,
      aux_sym_string_repeat2,
  [351] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(100), 3,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
  [360] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(102), 2,
      anon_sym_RBRACE,
      sym_identifier,
  [368] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(104), 1,
      sym_identifier,
    STATE(7), 1,
      sym_nested_identifier,
  [378] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(106), 2,
      anon_sym_RBRACE,
      sym_identifier,
  [386] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(108), 2,
      anon_sym_RBRACE,
      sym_identifier,
  [394] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(110), 2,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
  [402] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(112), 2,
      anon_sym_RBRACE,
      sym_identifier,
  [410] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(114), 2,
      anon_sym_RBRACE,
      sym_identifier,
  [418] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(116), 2,
      anon_sym_RBRACE,
      sym_identifier,
  [426] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(118), 2,
      anon_sym_RBRACE,
      sym_identifier,
  [434] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(120), 2,
      anon_sym_RBRACE,
      sym_identifier,
  [442] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(122), 2,
      anon_sym_RBRACE,
      sym_identifier,
  [450] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(124), 2,
      anon_sym_RBRACE,
      sym_identifier,
  [458] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(110), 2,
      anon_sym_RBRACE,
      sym_identifier,
  [466] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(126), 1,
      anon_sym_RBRACE,
  [473] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(128), 1,
      sym_identifier,
  [480] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(130), 1,
      anon_sym_RPAREN,
  [487] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(132), 1,
      anon_sym_COLON,
  [494] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(134), 1,
      anon_sym_RBRACE,
  [501] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(136), 1,
      anon_sym_LBRACE,
  [508] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(138), 1,
      anon_sym_RBRACE,
  [515] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(140), 1,
      ts_builtin_sym_end,
  [522] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(142), 1,
      ts_builtin_sym_end,
  [529] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(144), 1,
      anon_sym_RBRACE,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(3)] = 0,
  [SMALL_STATE(4)] = 31,
  [SMALL_STATE(5)] = 62,
  [SMALL_STATE(6)] = 93,
  [SMALL_STATE(7)] = 118,
  [SMALL_STATE(8)] = 136,
  [SMALL_STATE(9)] = 150,
  [SMALL_STATE(10)] = 168,
  [SMALL_STATE(11)] = 180,
  [SMALL_STATE(12)] = 192,
  [SMALL_STATE(13)] = 206,
  [SMALL_STATE(14)] = 220,
  [SMALL_STATE(15)] = 234,
  [SMALL_STATE(16)] = 247,
  [SMALL_STATE(17)] = 260,
  [SMALL_STATE(18)] = 273,
  [SMALL_STATE(19)] = 286,
  [SMALL_STATE(20)] = 299,
  [SMALL_STATE(21)] = 312,
  [SMALL_STATE(22)] = 325,
  [SMALL_STATE(23)] = 338,
  [SMALL_STATE(24)] = 351,
  [SMALL_STATE(25)] = 360,
  [SMALL_STATE(26)] = 368,
  [SMALL_STATE(27)] = 378,
  [SMALL_STATE(28)] = 386,
  [SMALL_STATE(29)] = 394,
  [SMALL_STATE(30)] = 402,
  [SMALL_STATE(31)] = 410,
  [SMALL_STATE(32)] = 418,
  [SMALL_STATE(33)] = 426,
  [SMALL_STATE(34)] = 434,
  [SMALL_STATE(35)] = 442,
  [SMALL_STATE(36)] = 450,
  [SMALL_STATE(37)] = 458,
  [SMALL_STATE(38)] = 466,
  [SMALL_STATE(39)] = 473,
  [SMALL_STATE(40)] = 480,
  [SMALL_STATE(41)] = 487,
  [SMALL_STATE(42)] = 494,
  [SMALL_STATE(43)] = 501,
  [SMALL_STATE(44)] = 508,
  [SMALL_STATE(45)] = 515,
  [SMALL_STATE(46)] = 522,
  [SMALL_STATE(47)] = 529,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_command, 2, .production_id = 3),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_nested_identifier, 3, .production_id = 7),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_func_def_repeat1, 2),
  [47] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_func_def_repeat1, 2), SHIFT_REPEAT(26),
  [50] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_command, 5, .production_id = 10),
  [52] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_command, 4, .production_id = 3),
  [54] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [56] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [58] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_object_repeat1, 2), SHIFT_REPEAT(41),
  [61] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_object_repeat1, 2),
  [63] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [65] = {.entry = {.count = 1, .reusable = false}}, SHIFT(19),
  [67] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [69] = {.entry = {.count = 1, .reusable = false}}, SHIFT(31),
  [71] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [73] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [75] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [77] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_repeat2, 2),
  [79] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_string_repeat2, 2), SHIFT_REPEAT(18),
  [82] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_command_repeat1, 2),
  [84] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_command_repeat1, 2), SHIFT_REPEAT(19),
  [87] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_repeat1, 2),
  [89] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_string_repeat1, 2), SHIFT_REPEAT(20),
  [92] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [94] = {.entry = {.count = 1, .reusable = false}}, SHIFT(30),
  [96] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [98] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [100] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object, 3, .production_id = 1),
  [102] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_def, 4),
  [104] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [106] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_def, 6, .production_id = 11),
  [108] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_def, 3, .production_id = 4),
  [110] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object, 2),
  [112] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3),
  [114] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 2),
  [116] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_def, 5, .production_id = 9),
  [118] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_def, 5, .production_id = 8),
  [120] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_definition, 3, .production_id = 2),
  [122] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_def, 4, .production_id = 6),
  [124] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_def, 3, .production_id = 5),
  [126] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [128] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [130] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [132] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [134] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [136] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [138] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [140] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [142] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [144] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
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
    .keyword_lex_fn = ts_lex_keywords,
    .keyword_capture_token = sym_identifier,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
