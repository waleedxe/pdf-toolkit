import { create } from "zustand";

import type { ToolId } from "../types/tools";

export type View = { type: "home" } | { type: "settings" } | { type: "tool"; toolId: ToolId };

interface NavigationState {
  view: View;
  goHome: () => void;
  goSettings: () => void;
  openTool: (toolId: ToolId) => void;
}

export const useNavigationStore = create<NavigationState>()((set) => ({
  view: { type: "home" },
  goHome: () => set({ view: { type: "home" } }),
  goSettings: () => set({ view: { type: "settings" } }),
  openTool: (toolId) => set({ view: { type: "tool", toolId } }),
}));
