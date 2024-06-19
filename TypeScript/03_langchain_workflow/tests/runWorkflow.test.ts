import { fetchStockData } from "../src/function/fetchStockData";
import type { AlpacaError } from "../src/types/error";
import { runWorkflow } from "../src/workflow/workflow";

// Mock fetchStockData
jest.mock("../src/function/fetchStockData");

describe("runWorkflow", () => {
  it("should run workflow and log stock metrics", async () => {
    const mockMetrics = { symbol: "DOUL", price: 100 };
    (fetchStockData as jest.Mock).mockResolvedValue(mockMetrics);

    console.log = jest.fn();
    await runWorkflow("DOUL");

    expect(console.log).toHaveBeenCalledWith("stockMetrics: ", mockMetrics);
  });

  it("should handle error and log message", async () => {
    const mockError = {
      message: JSON.stringify({
        code: 40110000,
        message: "request is not authorized",
      }),
    };
    (fetchStockData as jest.Mock).mockRejectedValue(mockError);

    console.error = jest.fn();
    await expect(runWorkflow("DOUL")).rejects.toThrow(
      "request is not authorized"
    );
    expect(console.error).toHaveBeenCalledWith(
      "Error running workflow:",
      "request is not authorized"
    );
  });
});
