class Product;
  local string code;
  local string name;
  local real price;

  function new(string name="", string code="", real price=0);
    this.name = name;
    this.code = code;
    this.price = price;
  endfunction

  function string get_code();
    return this.code;
  endfunction

  function void set_code(string code="");
    this.code = code;
  endfunction

  function string get_name();
    return this.name;
  endfunction

  function void set_name(string name="");
    this.name = name;
  endfunction

    function real get_price();
    return this.price;
  endfunction

  function void set_price(string price="");
    this.price = price;
  endfunction
endClass