import { runWorkflow } from "./workflow/workflow";

function main() {
  const args = process.argv.slice(2);

  if (args.length !== 1) {
    console.error("Usage: bun main.js <stock_symbol>");
    process.exit(1);
  }

  const symbol = args[0].toUpperCase();
  runWorkflow(symbol).catch((error) =>
    console.error("Error starting workflow:", error)
  );
}

main();
