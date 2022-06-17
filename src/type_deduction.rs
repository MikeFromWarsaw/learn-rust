fn main() {
    // Prosty i dobry opis zagadnienia składni turbofish można znaleźć tutaj
    // https://techblog.tonsser.com/posts/what-is-rusts-turbofish
    // Jednak całkiem wyczerpująco opowiada sam kompilator Rusta:
    // `rustc --explain E0282`
    
       
    let numbers : Vec<i32> = vec![1, 2, 3];
    
    // Use `turbofish` (::<type>) syntax is used to give a hint to the collect
    // function regarding a type of the variable it is supposed to return.
    // Generyczna funkcja collect potrzebuje informacji o typie zmiennej.
    // Co robi poniższy kod? 
    // 1 iter() - tworzy i zwraca iterator
    // 2 rev() - tworzy i zwraca iterator zwracający elementy w odwrotnej kolejności
    // 3 collect() - umieszcza elementy dostępne iteratorowi w wektorze i go zwraca.
    let result1 = numbers.iter().rev().collect::<Vec<&i32>>();
    println!("Odwrócony wektor {:?}", result1);
    
    // Alternatywnie możemy określić typ zmiennej resul2 w momencie jej deklaracji.
    // Dzięki temu collect użyje typu zmiennej result2 w celu utworzenia i zwrócenia
    // wymaganej kolekcji z elementami.
    let result2: Vec<&i32> = numbers.iter().rev().collect();
    println!("Odwrócony wektor {:?}", result2);

    // Znajdź, ile jest szóstek na koniec roku w klasie z danego przedmiotu. 
    let class_scores : Vec<i32> = vec![5, 3, 3, 3, 2, 6, 6];

    // Przy użyciu collect (nieefektywne) i składni turobfish.
    let scored  = class_scores
        .into_iter()
        .filter(|n| n == &6)
        .collect::<Vec<i32>>().len();

    println!("Liczna 6 w klasie to {:?}", scored);

    let class_scores_2 : Vec<i32> = vec![5, 3, 3, 3, 2, 6, 6];

    // Efektywna wersja.
    let scored_2  = class_scores_2
        .into_iter()
        .filter(|n| n == &6)
        .count();
    println!("Liczna 6 w klasie to {:?}", scored_2);
    
}
