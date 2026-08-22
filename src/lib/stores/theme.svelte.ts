export type Theme = "dark" | "light";

class ThemeStore {
  current = $state<Theme>("dark");

  constructor() {
    // Initial sync will happen onMount in browser
  }

  init() {
    if (typeof window === "undefined") return;

    const saved = localStorage.getItem("ldnoob-theme") as Theme | null;
    if (saved === "dark" || saved === "light") {
      this.current = saved;
    } else {
      const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
      this.current = prefersDark ? "dark" : "light";
    }

    this.applyTheme();

    // Listen for system theme changes if not overridden
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    mediaQuery.addEventListener("change", (e) => {
      const saved = localStorage.getItem("ldnoob-theme");
      if (!saved) {
        this.current = e.matches ? "dark" : "light";
        this.applyTheme();
      }
    });
  }

  toggle() {
    this.current = this.current === "dark" ? "light" : "dark";
    if (typeof window !== "undefined") {
      localStorage.setItem("ldnoob-theme", this.current);
      this.applyTheme();
    }
  }

  setTheme(theme: Theme) {
    this.current = theme;
    if (typeof window !== "undefined") {
      localStorage.setItem("ldnoob-theme", theme);
      this.applyTheme();
    }
  }

  private applyTheme() {
    if (typeof document === "undefined") return;
    const root = document.documentElement;
    if (this.current === "dark") {
      root.classList.add("dark");
      root.setAttribute("data-theme", "dark");
    } else {
      root.classList.remove("dark");
      root.setAttribute("data-theme", "light");
    }
  }
}

export const themeStore = new ThemeStore();
