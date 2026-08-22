import { invoke } from "@tauri-apps/api/core";
import type { ProxyRecord, ProxyCheckResult } from "$lib/types";
import { logsStore } from "./logs.svelte";

class ProxiesStore {
  proxies = $state<ProxyRecord[]>([]);
  isLoading = $state(false);

  async addProxy(proxyString: string) {
    // Parse host:port:user:pass or host:port
    const parts = proxyString.trim().split(":");
    if (parts.length < 2) {
      logsStore.error("Proxy", `Invalid proxy format: ${proxyString}`);
      return;
    }

    const host = parts[0];
    const port = parseInt(parts[1], 10);
    const username = parts[2] || undefined;
    const password = parts[3] || undefined;

    const record: ProxyRecord = {
      id: Math.random().toString(36).substring(2, 9),
      host,
      port,
      username,
      password,
      protocol: "socks5",
      status: "untested",
    };

    this.proxies = [record, ...this.proxies];
    logsStore.info("Proxy", `Added proxy ${host}:${port}`);
    await this.testProxy(record.id);
  }

  async testProxy(id: string) {
    const proxy = this.proxies.find((p) => p.id === id);
    if (!proxy) return;

    proxy.status = "testing";
    const proxyStr = `${proxy.host}:${proxy.port}`;

    try {
      const res = await invoke<ProxyCheckResult>("check_proxy", {
        proxyStr,
      });

      if (res.is_valid) {
        proxy.status = "active";
        proxy.latency_ms = res.latency_ms;
        proxy.ip = res.ip;
        proxy.country = res.country;
        proxy.city = res.city;
        logsStore.success(
          "Proxy",
          `Proxy ${proxy.host}:${proxy.port} is ACTIVE (${res.latency_ms}ms)`
        );
      } else {
        proxy.status = "error";
        proxy.latency_ms = res.latency_ms;
        logsStore.warn(
          "Proxy",
          `Proxy ${proxy.host}:${proxy.port} failed: ${res.error || "Timeout"}`
        );
      }
    } catch (e) {
      proxy.status = "error";
      logsStore.error("Proxy", `Proxy check error: ${e}`);
    }
  }

  removeProxy(id: string) {
    this.proxies = this.proxies.filter((p) => p.id !== id);
  }
}

export const proxiesStore = new ProxiesStore();
