/**
 * Smoke test to verify the SDK module structure.
 * This test verifies that the main SDK exports are accessible.
 */

describe("SDK Smoke Test", () => {
  it("should have a valid index module", () => {
    // This test verifies that the index module can be required
    // We use require to avoid TypeScript transpilation issues with broken imports
    expect(() => {
      require("../index");
    }).not.toThrow();
  });

  it("should export MerkleProofGenerator", () => {
    const SDK = require("../index");
    expect(SDK.MerkleProofGenerator).toBeDefined();
  });

  it("should export wallet adapters", () => {
    const SDK = require("../index");
    expect(SDK.FreighterAdapter).toBeDefined();
    expect(SDK.XBullAdapter).toBeDefined();
    expect(SDK.autoDetectWallet).toBeDefined();
  });

  it("should export hasher", () => {
    const SDK = require("../index");
    expect(SDK.UsernameHasher).toBeDefined();
    expect(SDK.bigintToBytes32).toBeDefined();
    expect(SDK.encodeUsername).toBeDefined();
    expect(SDK.hashUsername).toBeDefined();
  });

  it("should export resolver", () => {
    const SDK = require("../index");
    expect(SDK.UsernameResolver).toBeDefined();
  });

  it("should export register function", () => {
    const SDK = require("../index");
    expect(SDK.registerUsername).toBeDefined();
  });

  it("should export error classes", () => {
    const SDK = require("../index");
    expect(SDK.AlienGatewayError).toBeDefined();
    expect(SDK.NoAddressLinkedError).toBeDefined();
    expect(SDK.ProofGenerationError).toBeDefined();
    expect(SDK.TransactionFailedError).toBeDefined();
    expect(SDK.UsernameNotFoundError).toBeDefined();
    expect(SDK.UsernameUnavailableError).toBeDefined();
  });
});
