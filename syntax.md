`<var or const> <data_type> <variable_name> = <some_expression>`

examples:
```
    const u8 my_char = 128;
    var i32 my_int = 12345;
    var f64 my_float = 3.15159 * 2.71828;
    my_int++;
    my_int--;
```

Function declaration:
```
fn my_func(u8 some_char, f32 doogile) -> {
    print("the char is ${some_char}.");
    return(doogile + 12);
}
```

lambdas will have to wait for another time

Structs are complicated so they'll probably have to wait until the end since they're also not necessary for an MVP:
```
typedef struct VirtualMachine {
    u32[] registers,
    u32 pc,
    u32[] memory,
    u32 max_memory,
    u32 min_memory,

    fn read_register(self, u32 register_number) -> u32 {
        return(self.registers[register_number]);
    },
}
```

if, else, elif:
```
if(some_truth_value) {
    do_domething();
} elif(some_other_truth_value) {
    do_something_else();
} else {
    the_default_action();
}
```

for, while:
```
for(u32 i = 0; i < some_number; i++) {
    do_something();
}

while(some_truth_value) {
    do_something();
    modify(some_truth_value);
}
```

error handling is...
Maybe errors as values?
Worry about it later