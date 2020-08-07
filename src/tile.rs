pub enum Tile {
    /// ---
    Straight,
    /// \
    ///  \
    ///   \
    Diagonal,
    ///   |
    ///  /
    /// -
    Left90,
    /// -
    ///  \
    ///   |
    Right90,
    ///   /
    /// --
    Left45,
    /// --
    ///   \
    Right45,
    /// \
    ///  \
    /// ---
    Left135,
    /// ---
    //   /
    //  /
    Right135,
    Universal,
}
