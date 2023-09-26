use inkwell::{
    builder::Builder,
    context::{self, Context},
    execution_engine::JitFunction,
    module::Module,
    types::FloatMathType,
    values::AnyValue,
    values::FloatMathValue,
    OptimizationLevel,
};

use crate::heuristic::{
    executors::HeuristicExecuter,
    parser::{HeuristicNode, Rule},
    Heuristic,
};

type HeuristicFunc = unsafe extern "C" fn(f32, f32, f32, f32) -> f32;

struct Jit<'a> {
    context: &'a Context,
    module: Module<'a>,
    // function: JitFunction<'a, HeuristicFunc>,
}

// pre-initialize other LLVM steps? (first profile)

impl<'a> Jit<'a> {
    fn create(&self, heuristic: &Heuristic, context: &'a Context) -> i32 {
        let context = Context::create();
        let module = context.create_module("heuristic");

        let builder = context.create_builder();

        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::Default)
            .unwrap();

        let f32_type = context.f32_type();
        let fn_type = f32_type.fn_type(
            &[
                f32_type.into(),
                f32_type.into(),
                f32_type.into(),
                f32_type.into(),
            ],
            false,
        );

        let function = module.add_function("execute", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");

        builder.position_at_end(basic_block);

        let x1 = function.get_nth_param(0).unwrap().into_float_value();
        let y1 = function.get_nth_param(1).unwrap().into_float_value();
        let x2 = function.get_nth_param(2).unwrap().into_float_value();
        let y2 = function.get_nth_param(3).unwrap().into_float_value();

        // for node in ast {
        //     let recursive_builder = RecursiveBuilder::new(i32_type, &builder);
        //     let return_value = recursive_builder.build(&node);
        //     let _ = builder.build_return(Some(&return_value));
        // }

        let jit = Jit {
            context: &context,
            module,
            // function: unsafe { execution_engine.get_function("execute") }.unwrap(),
        };

        // jit
        1
    }

    // fn execute(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    //     unsafe { self.function.call(x1, y1, x2, y2) }
    // }
}

// fn build(
//     context: &Context,
//     builder: &Builder,
//     node: &HeuristicNode,
//     x1: FloatMathValue,
//     y1: FloatMathValue,
//     x2: FloatMathValue,
//     y2: FloatMathValue,
// ) -> FloatMathValue {
//     // match node {
//     //     HeuristicNode::Terminal(rule) => build_terminal(*rule, x1, y1, x2, y2),
//     //     HeuristicNode::Unary(rule, h) => {
//     //         build_unary(context, builder, module, *rule, &*h, x1, y1, x2, y2)
//     //     }
//     //     HeuristicNode::Binary(rule, h1, h2) => {
//     //         // build_binary(context, builder, module, *rule, &*h1, &*h2, x1, y1, x2, y2)
//     //         unreachable!("{:?}", rule);
//     //     }
//     // }
//     unreachable!("{:?}", rule);
// }

// fn build_terminal(
//     rule: Rule,
//     x1: FloatMathValue,
//     y1: FloatMathValue,
//     x2: FloatMathValue,
//     y2: FloatMathValue,
// ) -> FloatMathValue {
//     match rule {
//         Rule::x1 => x1,
//         Rule::y1 => y1,
//         Rule::x2 => x2,
//         Rule::y2 => y2,
//         Rule::deltaX => x2 - x1,
//         Rule::deltaY => y2 - y1,
//         _ => {
//             unreachable!("{:?}", rule);
//         }
//     }
// }

// fn build_unary(
//     context: &Context,
//     module: &Module,
//     builder: &Builder,
//     rule: Rule,
//     h: &HeuristicNode,
//     x1: FloatMathValue,
//     y1: FloatMathValue,
//     x2: FloatMathValue,
//     y2: FloatMathValue,
// ) -> FloatMathValue {
//     let result = build(context, builder, module, h, x1, y1, x2, y2);
//     match rule {
//         Rule::neg => -result,
//         Rule::abs => {
//             // builder
//         }
//         Rule::sqrt => result.signum() * result.abs().sqrt(),
//         Rule::sqr => result * result,
//         _ => {
//             unreachable!("{:?}", rule);
//         }
//     }
// }
