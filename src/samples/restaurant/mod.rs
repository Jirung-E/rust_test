/*
 * struct
 * enum
 * slice
 * closure
 */

pub fn test() {
    let mut restaurant1 = Restaurant::new("Hans");
    restaurant1.add_menu(Menu::Main(MenuItem::new("Chicken Omelette", 6000)));
    restaurant1.add_menu(Menu::Main(MenuItem::new("Beef Omelette", 7000)));
    restaurant1.add_menu(Menu::Main(MenuItem::new("Pork Omelette", 8000)));
    restaurant1.add_menu(Menu::Side(MenuItem::new("Fried Potato", 2000)));
    restaurant1.add_menu(Menu::Side(MenuItem::new("Chicken Nuggets(5 pieces)", 3000)));
    restaurant1.add_menu(Menu::Side(MenuItem::new("Chicken Nuggets(10 pieces)", 5000)));
    restaurant1.add_menu(Menu::Drink(MenuItem::new("Coke", 1000)));
    restaurant1.add_menu(Menu::Drink(MenuItem::new("Sprite", 1000)));
    restaurant1.add_menu(Menu::Drink(MenuItem::new("Water", 0)));
    restaurant1.print_menu();
}

struct Restaurant {
    name: String,
    main_menu: Vec<MenuItem>,
    side_menu: Vec<MenuItem>,
    drink_menu: Vec<MenuItem>
}

impl Restaurant {
    fn new(name: &str) -> Restaurant {
        Restaurant {
            name: String::from(name),
            main_menu: Vec::new(),
            side_menu: Vec::new(),
            drink_menu: Vec::new(),
        }
    }

    fn add_menu(&mut self, menu: Menu) {
        match menu {
            Menu::Main(item) => self.main_menu.push(item),
            Menu::Side(item) => self.side_menu.push(item),
            Menu::Drink(item) => self.drink_menu.push(item),
        }
    }

    fn print_menu(&self) {
        println!(" [ Restaurant {} ] ", self.name);

        let print_menu = |menu: &MenuItem| {
            let menu_name_max_length: usize = 25;
            println!("  - {:menu_name_max_length$} ..{}", menu.name, menu.price);
        };
        let print_menu = |menu: &Vec<MenuItem>| {
            for menu in menu.iter() {
                print_menu(menu);
            }
        };

        println!(" Main");
        print_menu(&self.main_menu);
        println!(" Side");
        print_menu(&self.side_menu);
        println!(" Drink");
        print_menu(&self.drink_menu);
    }
}

enum Menu {
    Main(MenuItem),
    Side(MenuItem),
    Drink(MenuItem)
}

struct MenuItem {
    name: String,
    price: u32,
}

impl MenuItem {
    fn new(name: &str, price: u32) -> MenuItem {
        let mut name = name;
        if name.len() > 25 {
            name = &name[..25];
        }
        MenuItem {
            name: String::from(name),
            price,
        }
    }
}
