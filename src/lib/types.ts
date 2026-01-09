interface Category {
    id: number,
    name: string,
    color: string,
    secondary_color: string,
    icon: string
}
type CategoryDetails =  Map<String, Category>;

export type { Category, CategoryDetails };

