//! 지금까지 본 소유권에 대한 설명 중에 가장 깔끔하다.
fn main() {
    // 가변 데이터를 생성한다.
    let mut top_grossing_films = vec!["Avatar", "Avengers: Endgame", "Titanic"];
    // 가변 참조를 생성한다.
    let top_grossing_films_mutable_reference = &mut top_grossing_films;
    // 가변 참조를 사용해서 데이터를 변경한다.
    top_grossing_films_mutable_reference.push("Star Wars: The Force Awakens");
    // 동일한 데이터의 불변 참조를 가져오므로 이전의 가변참조는 무효화 된다.
    let top_grossing_films_reference = &top_grossing_films;
    println!(
        "Printed using immutable reference: {:#?}",
        top_grossing_films_reference
    );
    // 소유권을 이전한다.
    let top_grossing_films_moved = top_grossing_films;
    println!("Printed after moving: {:#?}", top_grossing_films_moved);

    // 오류: 변수가 이동되어서 유효하지 않다.
    // println!("Print using original value: {:#?}", top_grossing_films);
    // 오류: 불변 참조를 만들 때 가변 참조가 무효화 됐다.
    // println!("Print using mutable reference: {:#?}",top_grossing_films_mutable_reference)
}
