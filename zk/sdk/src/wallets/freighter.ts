import type { WalletAdapter } from "./shared";
import { WalletDetectionError, assertBrowserEnvironment } from "./shared";

interface FreighterErrorResponse {
  error?: string;
}

interface FreighterAddressResponse {
  address?: string;
  publicKey?: string;
  error?: string;
}

interface FreighterSignResponse {
  signedTxXdr?: string;
  signedXDR?: string;
  xdr?: string;
  error?: string;
}

export interface FreighterApi {
  isConnected?(): Promise<boolean> | boolean;
  isAllowed?(): Promise<boolean> | boolean;
  requestAccess(): Promise<string | FreighterErrorResponse>;
  getPublicKey?(): Promise<string | FreighterAddressResponse>;
  getAddress?(): Promise<string | FreighterAddressResponse>;
  signTransaction(
    xdr: string,
    options?: Record<string, unknown>,
  ): Promise<string | FreighterSignResponse>;
}

type BrowserGlobal = typeof globalThis & { window?: unknown };

export class FreighterAdapter implements WalletAdapter {
  private publicKey?: string;

  public constructor(private readonly api?: FreighterApi) {}

  public static async isAvailable(): Promise<boolean> {
    if (typeof (globalThis as BrowserGlobal).window === "undefined") {
      return false;
    }

    try {
      await import("@stellar/freighter-api");
      return true;
    } catch {
      return false;
    }
  }

  public async connect(): Promise<void> {
    const freighter = await this.getApi();
    const response = await freighter.requestAccess();
    const publicKey = extractFreighterString(response, "Freighter access request failed.");

    if (!publicKey) {
      throw new WalletDetectionError("Freighter did not return a public key.");
    }

    this.publicKey = publicKey;
  }

  public async getPublicKey(): Promise<string> {
    if (this.publicKey) {
      return this.publicKey;
    }

    const freighter = await this.getApi();
    const addressResult = freighter.getPublicKey
      ? await freighter.getPublicKey()
      : freighter.getAddress
        ? await freighter.getAddress()
        : undefined;

    const publicKey = extractFreighterString(addressResult, "Unable to read the active Freighter public key.");

    if (!publicKey) {
      throw new WalletDetectionError("Freighter did not expose an active public key.");
    }

    this.publicKey = publicKey;
    return publicKey;
  }

  public async signTransaction(xdr: string): Promise<string> {
    const freighter = await this.getApi();
    const signed = await freighter.signTransaction(xdr);

    return extractSignedXdr(signed, "Freighter failed to sign the Soroban transaction.");
  }

  private async getApi(): Promise<FreighterApi> {
    if (this.api) {
      return this.api;
    }

    assertBrowserEnvironment();

    try {
      return await import("@stellar/freighter-api");
    } catch (error) {
      throw new WalletDetectionError("Freighter is not available in this browser.");
    }
  }
}

function extractFreighterString(
  value: string | FreighterErrorResponse | FreighterAddressResponse | undefined,
  fallbackMessage: string,
): string {
  if (typeof value === "string") {
    return value;
  }

  if (value?.error) {
    throw new WalletDetectionError(value.error);
  }

  const address = "address" in (value ?? {}) ? value.address : undefined;
  const publicKey = "publicKey" in (value ?? {}) ? value.publicKey : undefined;
  const result = address ?? publicKey;

  if (!result) {
    throw new WalletDetectionError(fallbackMessage);
  }

  return result;
}

function extractSignedXdr(
  value: string | FreighterSignResponse,
  fallbackMessage: string,
): string {
  if (typeof value === "string") {
    return value;
  }

  if (value.error) {
    throw new WalletDetectionError(value.error);
  }

  const signedXdr = value.signedTxXdr ?? value.signedXDR ?? value.xdr;
  if (!signedXdr) {
    throw new WalletDetectionError(fallbackMessage);
  }

  return signedXdr;
}
