fn main() {
   
  /*
    Scalar types:
        Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
        Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
        Floating point: f32, f64
        char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
        bool either true or false
        The unit type (), whose only possible value is an empty tuple: ()
   */
   println!("\nScalar types:\n");
   println!("Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)");
   let num: i8 = 127;
   println!("{0:0128b} {1:4}", num, "i8");

   let num: i16 = 32767;
   println!("{0:0128b} {1:4}", num, "i16");

   let num: i32 = 2_14_7483_647;
   println!("{0:0128b} {1:4}", num, "i32");

   let num: i64 = 9_223_372_036_854_775_807;
   println!("{0:0128b} {1:4}", num, "i64");

   let num: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
   println!("{0:0128b} {1:4}", num, "i128");

   let num: isize = 9_223_372_036_854_775_807;
   println!("{0:0128b} {1:4}", num, "isiz");

   println!("\nUnsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)");
   let num: u8 = 255;
   println!("{0:0128b} {1:4}", num, "u8");

   let num: u16 = 65_535;
   println!("{0:0128b} {1:4}", num, "u16");

   let num: u32 = 4_294_967_295;
   println!("{0:0128b} {1:4}", num, "u32");

   let num: u64 = 18_446_744_073_709_551_615;
   println!("{0:0128b} {1:4}", num, "u64");

   let num: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
   println!("{0:0128b} {1:4}", num, "u128");

   let num: usize = 18_446_744_073_709_551_615;
   println!("{0:0128b} {1:4}", num, "usiz");

   println!("\nFloating point: f32, f64");
   let num: f32 = 3.14159265;
   println!("{0:.8} {1:4}", num, "f32");

   let num: f64 = 3.14159265;
   println!("{0:.8} {1:4}", num, "f64");


   println!("\nchar Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)");
   let a: char = 'a';
   let b: char = 'b';
   let c: char = 'c';
   let o: char = 'o';
   println!("{1}{3}{2}{0}", a, b, c, o);


   println!("\nbool either true or false");
   let is_hot = true;
   let is_girlfriend = true;
   println!("Is she hot? {}", if is_hot { "Yes" } else { "No" });
   println!("Is she your girlfriend? {}", if is_girlfriend {"Yes"} else {"No"});


   println!("\nThe unit type (), whose only possible value is an empty tuple: ()");
   let unit_type: () = ();
   println!("The unit type: {:#?}", unit_type);

   /*
     Compound Types
        Arrays like [1, 2, 3]
        Tuples like (1, true)
    */
    let girls = ["Yor", "Nene", "Aia", "Nerissa", "Daki"];
    println!("Cute girls: {:?}", girls);

    let vtubers = ("Nene", "Aia", "Nerissa");
    println!("Cute vtubers: {:?}", vtubers);

}
