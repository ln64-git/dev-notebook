import { Asset, createClient } from "@alpacahq/typescript-sdk/esm/mod";
import { fetchStockData } from "../src/function/fetchStockData";

// Mock the Alpaca client
jest.mock("@alpacahq/typescript-sdk", () => {
  return {
    createClient: jest.fn().mockReturnValue({
      getAsset: jest.fn(),
    }),
  };
});

// Provide the necessary configuration for createClient
const mockClientOptions = {
  key: "test_key",
  secret: "test_secret",
};

const client = createClient(mockClientOptions) as jest.Mocked<
  ReturnType<typeof createClient>
>;

describe("fetchStockData", () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it("should fetch stock data successfully", async () => {
    const mockAsset: Asset = { symbol: "DOUL", name: "Double Ultra" } as Asset;
    client.getAsset.mockResolvedValue(mockAsset);

    const result = await fetchStockData("DOUL");
    expect(result).toEqual(mockAsset);
    expect(client.getAsset).toHaveBeenCalledWith({
      symbol_or_asset_id: "DOUL",
    });
  });

  it("should handle error and log message", async () => {
    const mockError = {
      message: JSON.stringify({
        code: 40110000,
        message: "request is not authorized",
      }),
    };
    client.getAsset.mockRejectedValue(mockError);

    console.error = jest.fn();
    await expect(fetchStockData("DOUL")).rejects.toThrow(
      "request is not authorized"
    );
    expect(console.error).toHaveBeenCalledWith(
      "Error fetching stock data:",
      "request is not authorized"
    );
  });
});
