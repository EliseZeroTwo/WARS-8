#[macro_use]
use lazy_static::lazy_static;

use std::collections::HashMap;

lazy_static! {
    pub static ref FONT: HashMap<char, &'static [[bool; 4]; 6]> =  {
        let mut m = HashMap::new();
        m.insert('A', &[
            [false, false, false, false], // 
            [false, true,  false, false], //  # 
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
        ]);
        m.insert('B', &[
            [false, false, false, false], // 
            [true,  true,  true,  false], // ### 
            [true,  false, true,  false], // # #
            [true,  true,  false, false], // ##
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
        ]);
        m.insert('C', &[
            [false, false, false, false], // 
            [false, true,  true,  false], //  ##
            [true,  false, false, false], // #
            [true,  false, false, false], // #
            [true,  false, false, false], // #
            [false, true,  true,  false], //  ##
        ]);
        m.insert('D', &[
            [false, false, false, false], // 
            [true,  true,  false, false], // ##
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  true,  false, false], // ##
        ]);
        m.insert('E', &[
            [false, false, false, false], // 
            [true,  true,  true,  false], // ###
            [true,  false, false, false], // #
            [true,  true,  false, false], // ##
            [true,  false, false, false], // #
            [true,  true,  true,  false], // ###
        ]);
        m.insert('F', &[
            [false, false, false, false], // 
            [true,  true,  true,  false], // ###
            [true,  false, false, false], // #
            [true,  true,  false, false], // ##
            [true,  false, false, false], // #
            [true,  false, false, false], // #
        ]);
        m.insert('G', &[
            [false, false, false, false], // 
            [true,  true,  true,  false], // ###
            [true,  false, false, false], // #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
        ]);
        m.insert('H', &[
            [false, false, false, false], // 
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
        ]);
        m.insert('I', &[
            [false, false, false, false], // 
            [true,  true,  true,  false], // ###
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [true,  true,  true,  false], // ###
        ]);
        m.insert('J', &[
            [false, false, false, false], // 
            [true,  true,  true,  false], // ###
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [true,  false, false, false], // #
        ]);
        m.insert('K', &[
            [false, false, false, false], // 
            [true,  false, true,  false], // # #
            [true,  true,  false, false], // ##
            [true,  true,  false, false], // ##
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
        ]);
        m.insert('L', &[
            [false, false, false, false], // 
            [true,  false, false, false], // #
            [true,  false, false, false], // #
            [true,  false, false, false], // #
            [true,  false, false, false], // #
            [true,  true,  true,  false], // ###
        ]);
        m.insert('M', &[
            [false, false, false, false], // 
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
        ]);
        m.insert('N', &[
            [false, false, false, false], // 
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
        ]);
        m.insert('O', &[
            [false, false, false, false], // 
            [true,  true,  false, false], // ##
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [false, true,  true,  false], //  ##
        ]);
        m.insert('P', &[
            [false, false, false, false], // 
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [true,  false, false, false], // #
            [true,  false, false, false], // #
        ]);
        m.insert('Q', &[
            [false, false, false, false], // 
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [false, false, false, true],  //    #
        ]);
        m.insert('R', &[
            [false, false, false, false], // 
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  true,  false, false], // ##
            [true,  false, true,  false], // # #
        ]);
        m.insert('S', &[
            [false, false, false, false], // 
            [true,  true,  true,  false], // ###
            [true,  false, false, false], // #
            [true,  true,  true,  false], // ###
            [false, false, true,  false], //   #
            [true,  true,  true,  false], // ###
        ]);
        m.insert('T', &[
            [false, false, false, false], // 
            [true,  true,  true,  false], // ###
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
        ]);
        m.insert('U', &[
            [false, false, false, false], // 
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
        ]);
        m.insert('V', &[
            [false, false, false, false], // 
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [false, true,  false, false], //  #
        ]);
        m.insert('W', &[
            [false, false, false, false], // 
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
        ]);
        m.insert('X', &[
            [false, false, false, false], // 
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [false, true,  false, false], //  #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
        ]);
        m.insert('Y', &[
            [false, false, false, false], // 
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
        ]);
        m.insert('Z', &[
            [false, false, false, false], // 
            [true,  true,  true,  false], // ###
            [false, false, true,  false], //   #
            [false, true,  false, false], //  #
            [true,  false, false, false], // #
            [true,  true,  true,  false], // ###
        ]);
        m.insert('1', &[
            [false, false, false, false], // 
            [true,  false, false, false], // #
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [true,  true,  true,  false], // ###
        ]);
        m.insert('2', &[
            [false, false, false, false], // 
            [true,  true,  false, false], // ##
            [false, false, true,  false], //   #
            [false, true,  false, false], //  #
            [true,  false, false, false], // #
            [false, true,  true,  false], //  ##
        ]);
        m.insert('3', &[
            [true,  false, false, false], // 
            [false, true,  true,  false], // ###
            [false, false, true,  false], //   #
            [false, true,  true,  false], // ###
            [false, false, true,  false], //   #
            [true,  true,  true,  false], // ###
        ]);
        m.insert('4', &[
            [false, false, false, false], // 
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [false, true,  true,  false], //  ##
            [false, false, true,  false], //   #
            [false, false, true,  false], //   #
        ]);
        m.insert('5', &[
            [false, false, false, false], // 
            [false, false, false, false], // ###
            [false, false, false, false], // #
            [false, false, false, false], //  #
            [false, false, false, false], //   #
            [false, false, false, false], // ##
        ]);
        m.insert('6', &[
            [false, false, false, false], // 
            [true,  true,  true,  false], // ###
            [true,  false, false, false], // #
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
        ]);
        m.insert('7', &[
            [false, false, false, false], // 
            [true,  true,  false, false], // ##
            [false, false, true,  false], //   #
            [false, false, true,  false], //   #
            [false, false, true,  false], //   #
            [false, false, true,  false], //   #
        ]);
        m.insert('8', &[
            [false, false, false, false], // 
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
        ]);
        m.insert('9', &[
            [false, false, false, false], // 
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [false, false, true,  false], //   #
            [false, false, true,  false], //   #
        ]);
        m.insert('0', &[
            [false, false, false, false], // 
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
        ]);
        m.insert('/', &[
            [false, false, false, false], // 
            [false, false, true, false], //   #
            [false, true, false, false], //  #
            [false, true, false, false], //  #
            [false, true, false, false], //  #
            [true, false, false, false], // #
        ]);
        m.insert('\\', &[
            [false, false, false, false], // 
            [true, false, false, false], // #
            [false, true, false, false], //  #
            [false, true, false, false], //  #
            [false, true, false, false], //  #
            [false, false, true, false], //   #
        ]);
        m.insert(',', &[
            [false, false, false, false], // 
            [false, false, false, false], // 
            [false, false, false, false], // 
            [false, false, false, false], // 
            [false, true,  false, false], //  #
            [true,  false, false, false], // #
        ]);
        m.insert('.', &[
            [false, false, false, false], // 
            [false, false, false, false], // 
            [false, false, false, false], // 
            [false, false, false, false], // 
            [true,  true,  false, false], // ##
            [true,  true,  false, false], // ##
        ]);
        m
    };
}