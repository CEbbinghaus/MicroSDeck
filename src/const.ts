export const PORT: number = 12412;
export const HOST: string = "localhost";
export const PROTOCOL: string = "http";

export const API_URL: string = `${PROTOCOL}://${HOST}${PORT ? (":" + PORT) : ""}`;

export const CONFIGURATION_PATH = "/microsdeck/config";
export const DOCUMENTATION_PATH = "/microsdeck/docs";

export const UNNAMED_CARD_NAME = "Unnamed Card";
