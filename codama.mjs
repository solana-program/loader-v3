import { getToolchainArgument } from "./scripts/utils.mjs";
import path from "node:path";
import fs from "node:fs";

const prettier = JSON.parse(
  fs.readFileSync(path.join("clients", "js", ".prettierrc.json"), "utf-8")
);

export default {
  idl: "program/idl.json",
  before: [
    {
      from: "codama#updateProgramsVisitor",
      args: [
        {
          solanaLoaderV3Program: {
            name: "loaderV3",
            publicKey: "BPFLoaderUpgradeab1e11111111111111111111111",
          },
        },
      ],
    },
  ],
  scripts: {
    js: {
      from: "@codama/renderers-js",
      args: ["clients/js/src/generated", { prettier }],
    },
    rust: {
      from: "@codama/renderers-rust",
      args: [
        "clients/rust/src/generated",
        {
          crateFolder: "clients/rust",
          formatCode: true,
          toolchain: getToolchainArgument("format"),
        },
      ],
    },
  },
};
