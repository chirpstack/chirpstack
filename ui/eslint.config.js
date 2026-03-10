// @ts-check

import { defineConfig } from "eslint/config";
import js from "@eslint/js";
import ts from "typescript-eslint";
import react from "eslint-plugin-react";
import reactHooks from "eslint-plugin-react-hooks";

export default defineConfig({
  extends: [
    js.configs.recommended,
    ts.configs.recommendedTypeChecked,
    reactHooks.configs.flat.recommended,
    react.configs.flat.recommended,
    react.configs.flat["jsx-runtime"],
  ],
  files: ["**/*.{ts,tsx}"],
  ignores: ["build/**"],
  settings: {
    react: {
      version: "19",
    },
  },
  languageOptions: {
    parserOptions: {
      projectService: true,
    },
  },
  rules: {
    "@typescript-eslint/consistent-type-imports": "error",
    "@typescript-eslint/no-unused-vars": "off",
    "@typescript-eslint/no-deprecated": "error",
  },
});
