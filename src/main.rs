mod gradvalue {
    pub type BackwardFn = fn(&mut GradValue, &mut GradValue, &GradValue);
    pub type ForwardFn = fn(&mut GradValue, grad_values: &[GradValue]);

    pub struct GradValue {
        pub name: String,
        pub data: f32,
        pub grad: f32,
        pub children: Option<Vec<usize>>,
        pub backward: Option<BackwardFn>,
        pub op: Option<String>,
    }

    impl GradValue {
        pub fn leaf_node(name: &str, data: f32) -> GradValue {
            GradValue { name: name.to_string(), data, grad: 0.0, children: None, backward: None, op: None }
        }

        pub fn get_node(grad_values: &[GradValue], offset: usize) -> &GradValue {
            &grad_values[offset]
        }

        fn func_node(grad_values: &[GradValue], data: f32, op: String,
                         children: &[usize], backward: BackwardFn) -> GradValue {
            let mut c_names = Vec::new();
            for c in children {
                let node = GradValue::get_node(grad_values, *c);
                c_names.push(node.name.clone());
            }
            let name = op.clone() + &c_names.join("_");
            GradValue { name, data, op: Some(op), grad: 0.0, children: Some(children.to_vec()), backward: Some(backward) }
        }


/*        fn call_forward(&mut self) {
            match self.forward {
                Some(forward) => forward(&mut self),
                None => todo!(),
            }
        }
*/
        pub fn add_op_node(grad_values: &[GradValue], inputs: &[usize]) -> GradValue {
            let mut total = 0.0;
            for i in inputs.iter() {
                let node = GradValue::get_node(grad_values, *i);
                total += node.data;
            }
            let op_node = GradValue::func_node(grad_values, total, "+".to_string(), inputs, GradValue::add);
            op_node
        }

        pub fn mult_op_node(grad_values: &[GradValue], inputs: &[usize]) -> GradValue {
            let mut total = 1.0;
            for i in inputs.iter() {
                let node = GradValue::get_node(grad_values, *i);
                total *= node.data;
            }
            let op_node = GradValue::func_node(grad_values, total, "*".to_string(), inputs, GradValue::mult);
            op_node
        }
/*
        pub fn add_node(&mut self, grad_values: &[GradValue], data: f32) {
            let child = GradValue::leaf_node(self.name.as_str(), data);
            self.children = Some(vec![child])
        }

        pub fn mod_node(&mut self, grad_values: &[GradValue], data: f32) {
            for c in &mut self.children.iter_mut().flatten() {
                let node = GradValue::get_node(grad_values, *c);
                node.data = data;
            }
        }
*/
        pub fn fn_negate(self_value: &mut GradValue, grad_values: &[GradValue]) {
            let mut total = 0.0;
            for i in self_value.children.iter().flatten() {
                let node = GradValue::get_node(grad_values, *i);
                total -= node.data;
            }
            self_value.data = total;
        }

        pub fn add(self_value: &mut GradValue, other_value: &mut GradValue, out_value: & GradValue) {
            self_value.grad = self_value.data + out_value.grad;
            other_value.grad = other_value.data + out_value.grad;
        }

        pub fn mult(self_value: &mut GradValue, other_value: &mut GradValue, out_value: & GradValue) {
            self_value.grad = self_value.data + (other_value.data * out_value.grad);
            other_value.grad = other_value.data + (self_value.data * out_value.grad);
        }

        pub fn print_node(& self, grad_values: &[GradValue]) {
            println!("node name {0} - data {1}, grad {2}", self.name, self.data, self.grad);
            for c in self.children.iter().flatten() {
                let node = GradValue::get_node(grad_values, *c);
                println!("c data {0}, grad {1}", node.data, node.grad);
            }
        }

//        fn get_value(grad_values: &Vec<GradValue>, name: &str) -> Option<GradValue> {
//            let name_str = name.to_string();
//            let value = grad_values.iter().find(|v| v.name == name_str);
//            value
//        }

    }


}

use crate::gradvalue::*;

fn main() {
    let mut grad_values: Vec<GradValue> = Vec::with_capacity(32);
    grad_values.push(GradValue::leaf_node("a", 2.0));  // a
    grad_values.push(GradValue::leaf_node("b", -3.0)); // b 
    grad_values.push(GradValue::mult_op_node(&grad_values[..], &[0, 1])); // e
    grad_values.push(GradValue::leaf_node("c", 10.0)); // c
    grad_values.push(GradValue::add_op_node(&grad_values[..], &[2, 3])); // d
    grad_values.push(GradValue::leaf_node("f", -2.0)); // f
    grad_values.push(GradValue::mult_op_node(&grad_values[..], &[4, 5])); // L
    grad_values[6].print_node(&grad_values);
//    a.add_node(-3.0);
//    a.mod_node(-4.0);
//    let mut c = GradValue::add_op_node(vec![GradValue::leaf_node(2.0), GradValue::leaf_node(3.0)]);
//    c = GradValue::add_op_node(vec![c, GradValue::leaf_node(1.0)]);
//    c.print_node();
//    a.data = -4.0;
//    c.print_node();
//    e.print_node();
}
