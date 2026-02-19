/**
 * https://nextjs.org/docs/app/api-reference/config/eslint
 */

import typescriptParser from "@typescript-eslint/parser"
import nextVitals from "eslint-config-next/core-web-vitals"
import nextTs from "eslint-config-next/typescript"
import prettier from "eslint-config-prettier/flat"
import perfectionist from "eslint-plugin-perfectionist"
import prettierPlugin from "eslint-plugin-prettier"
import unusedImportsPlugin from "eslint-plugin-unused-imports"
import { defineConfig, globalIgnores } from "eslint/config"

const eslintConfig = defineConfig([
  ...nextVitals,
  ...nextTs,
  prettier,
  {
    files: ["**/*.{ts,tsx}"],
    languageOptions: {
      parser: typescriptParser,
      parserOptions: {
        ecmaVersion: "latest",
        sourceType: "module",
        ecmaFeatures: { jsx: true },
      },
    },
    plugins: {
      /* Next.js provides: @typescript-eslint, react, react-hooks, import */
      perfectionist,
      prettier: prettierPlugin,
      "unused-imports": unusedImportsPlugin,
    },
    rules: {
      "@typescript-eslint/array-type": [
        "warn",
        {
          default: "array",
        },
      ],
      "@typescript-eslint/adjacent-overload-signatures": "off",
      "@typescript-eslint/method-signature-style": ["error", "property"],
      "@typescript-eslint/no-explicit-any": "error",
      "@typescript-eslint/no-unused-vars": "off",
      "@typescript-eslint/no-use-before-define": "error",
      "arrow-body-style": ["error", "as-needed"],
      eqeqeq: "error",
      "eslint-plugin-react/jsx-sort-props": "off",
      "import/no-unused-modules": "error",
      "import/order": "off" /* Prettier handles import sorting */,
      "no-await-in-loop": "error",
      "no-console": "warn",
      "no-return-await": "error",
      "no-var": "warn",
      "object-shorthand": "warn",
      "perfectionist/sort-interfaces": [
        "warn",
        {
          type: "alphabetical",
          order: "asc",
          groups: ["reserved", "data", "aria", "unknown", "callback"],
          customGroups: [
            {
              groupName: "reserved",
              elementNamePattern: "^(children|key|ref)$",
            },
            {
              groupName: "data",
              elementNamePattern: "^data-",
            },
            {
              groupName: "aria",
              elementNamePattern: "^aria-|role",
            },
            {
              groupName: "callback",
              elementNamePattern: "^on[A-Z]",
            },
          ],
        },
      ],
      "perfectionist/sort-jsx-props": [
        "warn",
        {
          type: "alphabetical",
          order: "asc",
          groups: ["reserved", "data", "aria", "unknown", "callback"],
          customGroups: [
            {
              groupName: "reserved",
              elementNamePattern: "^(children|key|ref)$",
            },
            {
              groupName: "data",
              elementNamePattern: "^data-",
            },
            {
              groupName: "aria",
              elementNamePattern: "^aria-|role",
            },
            {
              groupName: "callback",
              elementNamePattern: "^on[A-Z]",
            },
          ],
        },
      ],
      "prefer-const": "warn",
      "prettier/prettier": "warn",
      "quote-props": ["warn", "as-needed"],
      "react/jsx-filename-extension": [
        "warn",
        {
          extensions: ["ts", "tsx"],
        },
      ],
      "react/jsx-fragments": ["warn", "syntax"],
      "react/no-unused-prop-types": "error",
      "react/prop-types": "off",
      "react/react-in-jsx-scope": "off",
      "react-hooks/exhaustive-deps": "warn",
      "react-hooks/rules-of-hooks": "error",
      "unused-imports/no-unused-imports": "error",
      "unused-imports/no-unused-vars": [
        "warn",
        {
          vars: "all",
          varsIgnorePattern: "^_",
          args: "all",
          argsIgnorePattern: "^_",
        },
      ],
    },
    settings: {
      react: { version: "detect" },
      "import/resolver": {
        typescript: {},
      },
    },
  },
  /* Override default ignores of eslint-config-next. */
  globalIgnores([
    /* AI */
    ".agents",
    ".cursor",
    ".claude",
    /* Cloudflare */
    "cloudflare-env.d.ts",
    /* Testing */
    "coverage/**",
    /* Default ignores of eslint-config-next */
    ".next/**",
    "out/**",
    "build/**",
    "next-env.d.ts",
  ]),
])

export default eslintConfig
