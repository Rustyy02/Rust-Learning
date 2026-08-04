fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        let nice_slice = &a[1..4];

        // &a llama al objeto, en este caso array y [1..4] toma el indice 1 (2) hasta el 3 (el limite superior no se considera)

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???

        assert_eq!([2, 3, 4], nice_slice);
    }
}
