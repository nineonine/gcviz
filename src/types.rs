enum Ptr {
    Derived(i32, i32),
    Interior(i32, i32),
}

enum Reference {
    Pointer(Ptr),
    Null,
}
enum Field {
    Ref(Reference),
    Scalar(i32),
}
struct Object {
    size: i32,
    fields: Vec<Field> // slots
}

enum ExecutionFrame {
    Allocate(Object),
    Mutate(Object),
    RunGC,
}

type Word = i64;

enum WordStatus {
    Allocated, Freed, Wasted, Unusable
}

type Chunk = Vec<Word>;

type Cell = Vec<Word>;

type Block = Vec<Word>;

struct Heap {
    arr: Vec<Word>,
}

struct VM {
    heap_size: i32,
    heap: Heap,
}
