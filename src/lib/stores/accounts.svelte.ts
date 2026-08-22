import type { FacebookAccount } from "$lib/types";
import { logsStore } from "./logs.svelte";

class AccountsStore {
  accounts = $state<FacebookAccount[]>([]);
  selectedUids = $state<string[]>([]);

  addAccount(account: FacebookAccount) {
    this.accounts = [account, ...this.accounts.filter((a) => a.uid !== account.uid)];
    logsStore.info("Accounts", `Added account UID ${account.uid}`);
  }

  batchImport(lines: string[]) {
    let count = 0;
    for (const line of lines) {
      const parts = line.trim().split("|");
      if (parts.length >= 2) {
        const uid = parts[0].trim();
        const pass = parts[1].trim();
        const twoFA = parts[2]?.trim() || "";
        const cookie = parts[3]?.trim() || "";

        this.addAccount({
          profileKey: uid,
          emuIndex: -1,
          uid,
          pass,
          twoFA,
          cookie,
          status: "idle",
        });
        count++;
      }
    }
    logsStore.success("Accounts", `Successfully imported ${count} accounts`);
  }

  removeAccount(uid: string) {
    this.accounts = this.accounts.filter((a) => a.uid !== uid);
    this.selectedUids = this.selectedUids.filter((u) => u !== uid);
  }
}

export const accountsStore = new AccountsStore();
