import { runWorkflow } from "../src/workflow/workflow";

// Mock runWorkflow
jest.mock("../src/workflow/workflow");

describe("main", () => {
  const OLD_ENV = process.env;

  beforeEach(() => {
    jest.resetModules(); // Clears the cache
    process.env = { ...OLD_ENV };
  });

  afterEach(() => {
    process.env = OLD_ENV; // Restore old environment
  });

  it("should log usage error when no symbol is provided", () => {
    process.argv = ["node", "main.js"];
    console.error = jest.fn();

    require("../src/main");

    expect(console.error).toHaveBeenCalledWith(
      "Usage: bun main.js <stock_symbol>"
    );
    expect(process.exit).toHaveBeenCalledWith(1);
  });

  it("should run workflow and handle error", async () => {
    process.argv = ["bun", "src/main.js", "DOUL"];
    const mockError = {
      message: JSON.stringify({
        code: 40110000,
        message: "request is not authorized",
      }),
    };
    (runWorkflow as jest.Mock).mockRejectedValue(mockError);

    console.error = jest.fn();
    await require("../src/main");

    expect(console.error).toHaveBeenCalledWith(
      "Error starting workflow:",
      "request is not authorized"
    );
  });

  it("should run workflow successfully", async () => {
    process.argv = ["bun", "src/main.js", "DOUL"];
    (runWorkflow as jest.Mock).mockResolvedValue({});

    console.error = jest.fn();
    await require("../src/main");

    expect(console.error).not.toHaveBeenCalled();
  });
});
