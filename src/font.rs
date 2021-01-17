use std::collections::HashMap;

lazy_static! {
    pub static ref FONT: HashMap<char, &'static [[bool; 4]; 6]> =  {
        let mut m = HashMap::new();
        m.insert('A', &[
            [false, true,  false, false], //  #
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [false, false, false, false], //
        ]);
        m.insert('B', &[
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  true,  false, false], // ##
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [false, false, false, false], //
        ]);
        m.insert('C', &[
            [false, true,  true,  false], //  ##
            [true,  false, false, false], // #
            [true,  false, false, false], // #
            [true,  false, false, false], // #
            [false, true,  true,  false], //  ##
            [false, false, false, false], //
        ]);
        m.insert('D', &[
            [true,  true,  false, false], // ##
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  true,  false, false], // ##
            [false, false, false, false], //
        ]);
        m.insert('E', &[
            [true,  true,  true,  false], // ###
            [true,  false, false, false], // #
            [true,  true,  false, false], // ##
            [true,  false, false, false], // #
            [true,  true,  true,  false], // ###
            [false, false, false, false], //
        ]);
        m.insert('F', &[
            [true,  true,  true,  false], // ###
            [true,  false, false, false], // #
            [true,  true,  false, false], // ##
            [true,  false, false, false], // #
            [true,  false, false, false], // #
            [false, false, false, false], //
        ]);
        m.insert('G', &[
            [true,  true,  true,  false], // ###
            [true,  false, false, false], // #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [false, false, false, false], //
        ]);
        m.insert('H', &[
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [false, false, false, false], //
        ]);
        m.insert('I', &[
            [true,  true,  true,  false], // ###
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [true,  true,  true,  false], // ###
            [false, false, false, false], //
        ]);
        m.insert('J', &[
            [true,  true,  true,  false], // ###
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [true,  false, false, false], // #
            [false, false, false, false], //
        ]);
        m.insert('K', &[
            [true,  false, true,  false], // # #
            [true,  true,  false, false], // ##
            [true,  true,  false, false], // ##
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [false, false, false, false], //
        ]);
        m.insert('L', &[
            [true,  false, false, false], // #
            [true,  false, false, false], // #
            [true,  false, false, false], // #
            [true,  false, false, false], // #
            [true,  true,  true,  false], // ###
            [false, false, false, false], //
        ]);
        m.insert('M', &[
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [false, false, false, false], //
        ]);
        m.insert('N', &[
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [false, false, false, false], //
        ]);
        m.insert('O', &[
            [true,  true,  false, false], // ##
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [false, true,  true,  false], //  ##
            [false, false, false, false], //
        ]);
        m.insert('P', &[
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [true,  false, false, false], // #
            [true,  false, false, false], // #
            [false, false, false, false], //
        ]);
        m.insert('Q', &[
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [false, false, false, true],  //    #
            [false, false, false, false], //
        ]);
        m.insert('R', &[
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  true,  false, false], // ##
            [true,  false, true,  false], // # #
            [false, false, false, false], //
        ]);
        m.insert('S', &[
            [true,  true,  true,  false], // ###
            [true,  false, false, false], // #
            [true,  true,  true,  false], // ###
            [false, false, true,  false], //   #
            [true,  true,  true,  false], // ###
            [false, false, false, false], //
        ]);
        m.insert('T', &[
            [true,  true,  true,  false], // ###
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [false, false, false, false], //
        ]);
        m.insert('U', &[
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [false, false, false, false], //
        ]);
        m.insert('V', &[
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [false, true,  false, false], //  #
            [false, false, false, false], //
        ]);
        m.insert('W', &[
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [false, false, false, false], //
        ]);
        m.insert('X', &[
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [false, true,  false, false], //  #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [false, false, false, false], //
        ]);
        m.insert('Y', &[
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [false, false, false, false], //
        ]);
        m.insert('Z', &[
            [true,  true,  true,  false], // ###
            [false, false, true,  false], //   #
            [false, true,  false, false], //  #
            [true,  false, false, false], // #
            [true,  true,  true,  false], // ###
            [false, false, false, false], //
        ]);
        m.insert('1', &[
            [true,  false, false, false], // #
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [false, true,  false, false], //  #
            [true,  true,  true,  false], // ###
            [false, false, false, false], //
        ]);
        m.insert('2', &[
            [true,  true,  false, false], // ##
            [false, false, true,  false], //   #
            [false, true,  false, false], //  #
            [true,  false, false, false], // #
            [false, true,  true,  false], //  ##
            [false, false, false, false], //
        ]);
        m.insert('3', &[
            [true,  true,  true,  false], // ###
            [false, false, true,  false], //   #
            [false, true,  true,  false], // ###
            [false, false, true,  false], //   #
            [true,  true,  true,  false], // ###
            [false, false, false, false], //
        ]);
        m.insert('4', &[
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [false, true,  true,  false], //  ##
            [false, false, true,  false], //   #
            [false, false, true,  false], //   #
            [false, false, false, false], //
        ]);
        m.insert('5', &[
            [true,  true,  true,  false], // ###
            [true, false, false, false], // #
            [false, true, false, false], //  #
            [false, false, true, false], //   #
            [true, true, false, false], // ##
            [false, false, false, false], //
        ]);
        m.insert('6', &[
            [true,  true,  true,  false], // ###
            [true,  false, false, false], // #
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [false, false, false, false], //
        ]);
        m.insert('7', &[
            [true,  true,  false, false], // ##
            [false, false, true,  false], //   #
            [false, false, true,  false], //   #
            [false, false, true,  false], //   #
            [false, false, true,  false], //   #
            [false, false, false, false], //
        ]);
        m.insert('8', &[
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [false, false, false, false], //
        ]);
        m.insert('9', &[
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [false, false, true,  false], //   #
            [false, false, true,  false], //   #
            [false, false, false, false], //
        ]);
        m.insert('0', &[
            [true,  true,  true,  false], // ###
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  false, true,  false], // # #
            [true,  true,  true,  false], // ###
            [false, false, false, false], //
        ]);
        m.insert('/', &[
            [false, false, true,  false], //   #
            [false, true, false,  false], //  #
            [false, true, false,  false], //  #
            [false, true, false,  false], //  #
            [true, false, false,  false], // #
            [false, false, false, false], //
        ]);
        m.insert('\\', &[
            [true, false, false,  false], // #
            [false, true, false,  false], //  #
            [false, true, false,  false], //  #
            [false, true, false,  false], //  #
            [false, false, true,  false], //   #
            [false, false, false, false], //
        ]);
        m.insert(',', &[
            [false, false, false, false], //
            [false, false, false, false], //
            [false, false, false, false], //
            [false, true,  false, false], //  #
            [true,  false, false, false], // #
            [false, false, false, false], //
        ]);
        m.insert('.', &[
            [false, false, false, false], //
            [false, false, false, false], //
            [false, false, false, false], //
            [true,  true,  false, false], // ##
            [true,  true,  false, false], // ##
            [false, false, false, false], //
        ]);
        m
    };
}
