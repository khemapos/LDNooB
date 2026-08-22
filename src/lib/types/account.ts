export type AccountStatus = "idle" | "importing" | "logged_in" | "checkpoint" | "error";

export interface FacebookAccount {
  profileKey: string;
  emuIndex: number;
  uid: string;
  username?: string;
  pass: string;
  twoFA: string;
  proxy?: string;
  cookie?: string;
  token?: string;
  email?: string;
  emailPass?: string;
  phone?: string;
  userAgent?: string;
  status: AccountStatus;
  notes?: string;
}

export interface BatchImportRecord {
  uid: string;
  pass: string;
  twoFA?: string;
  cookie?: string;
  proxy?: string;
  email?: string;
}
