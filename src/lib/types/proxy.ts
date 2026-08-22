export type ProxyProtocol = "http" | "https" | "socks5" | "socks4";

export interface ProxyRecord {
  id: string;
  host: string;
  port: number;
  username?: string;
  password?: string;
  protocol: ProxyProtocol;
  latency_ms?: number;
  status: "untested" | "active" | "error" | "testing";
  country?: string;
  city?: string;
  ip?: string;
  assignedEmulatorIndex?: number;
}

export interface ProxyCheckResult {
  is_valid: boolean;
  ip?: string;
  country?: string;
  city?: string;
  timezone?: string;
  latency_ms: number;
  error?: string;
}

export interface IpData {
  ip: string;
  country: string;
  country_code: string;
  city: string;
  timezone: string;
  carrier?: string;
}
