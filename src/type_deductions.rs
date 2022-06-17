fn main() {
    let numbers : Vec<i32> = vec![1, 2, 3];
    
    // Use `turbofish` (::<type>) syntax is used to give a hint to the collect
    // function regarding a type of the variable it is supposed to return.
    // Generyczna funkcja collect potrzebuje informacji o typie zmiennej.
    // Co robi poniższy kod? 
    // 1 iter() - tworzy i zwraca iterator
    // 2 rev() - tworzy i zwraca iterator zwracający elementy w odwrotnej kolejności
    // 3 collect() - umieszcza elementy dostępne iteratorowi w wektorze i go zwraca.
    let result1 = numbers.iter().rev().collect::<Vec<&i32>>();
    println!("{:?}", result);
    
    // Alternatywnie możemy określić typ zmiennej result2 w momencie jej deklaracji.
    // Dzięki temu collect użyje typu zmiennej result2 w celu utworzenia i zwrócenia
    // wymaganej kolekcji z elementami.
    let result2: Vec<&i32> = numbers.iter().rev().collect();
    println!("{:?}", result2);
}
