interface FullCategory {
  id: number;
  name: string;
  color: string;
  icon?: string;
  budget: number | null;
  spent: number;
}

export type { FullCategory };
