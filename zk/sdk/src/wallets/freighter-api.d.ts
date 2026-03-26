declare module "@stellar/freighter-api" {
  const freighterApi: {
    isConnected?: () => Promise<boolean> | boolean;
    isAllowed?: () => Promise<boolean> | boolean;
    requestAccess: () => Promise<unknown>;
    getPublicKey?: () => Promise<unknown>;
    getAddress?: () => Promise<unknown>;
    signTransaction: (xdr: string, options?: Record<string, unknown>) => Promise<unknown>;
  };

  export = freighterApi;
}
