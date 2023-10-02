#![allow(clippy::too_many_arguments)]

use inkwell::intrinsics::Intrinsic;
use inkwell::{
    builder::Builder, context::Context, execution_engine::JitFunction, types::FloatType,
    values::FloatValue, values::FunctionValue, OptimizationLevel,
};

use crate::heuristic::{
    parser::{HeuristicNode, Rule},
    Heuristic,
};

type HeuristicFunc = unsafe extern "C" fn(f32, f32, f32, f32) -> f32;

pub struct Jit<'a> {
    // context: Context,
    // module: Module<'a>,
    function: JitFunction<'a, HeuristicFunc>,
}

// pre-initialize other LLVM steps? (first profile)

impl<'a> Jit<'a> {
    pub fn create(heuristic: &Heuristic, context: &'a Context) -> Self {
        // let context = context::Context::create();
        // let mut module: Module;
        let module = context.create_module("heuristic");

        let builder = context.create_builder();

        // Less is just as good as Aggressive, but 3x slower to compile than None (about 50% faster runtime performance)
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
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

        let abs_intrinsic = Intrinsic::find("llvm.fabs.f32").unwrap();
        assert!(abs_intrinsic.get_declaration(&module, &[]).is_none());
        let abs_fn = abs_intrinsic
            .get_declaration(&module, &[context.f32_type().into()])
            .unwrap();

        let copysign_intrinsic = Intrinsic::find("llvm.copysign.f32").unwrap();
        assert!(copysign_intrinsic.get_declaration(&module, &[]).is_none());
        let copysign_fn = copysign_intrinsic
            .get_declaration(
                &module,
                &[context.f32_type().into(), context.f32_type().into()],
            )
            .unwrap();

        let sqrt_intrinsic = Intrinsic::find("llvm.sqrt.f32").unwrap();
        assert!(sqrt_intrinsic.get_declaration(&module, &[]).is_none());
        let sqrt_fn = sqrt_intrinsic
            .get_declaration(&module, &[context.f32_type().into()])
            .unwrap();

        let min_intrinsic = Intrinsic::find("llvm.minnum.f32").unwrap();
        assert!(min_intrinsic.get_declaration(&module, &[]).is_none());
        let min_fn = min_intrinsic
            .get_declaration(
                &module,
                &[context.f32_type().into(), context.f32_type().into()],
            )
            .unwrap();

        let max_intrinsic = Intrinsic::find("llvm.maxnum.f32").unwrap();
        assert!(max_intrinsic.get_declaration(&module, &[]).is_none());
        let max_fn = max_intrinsic
            .get_declaration(
                &module,
                &[context.f32_type().into(), context.f32_type().into()],
            )
            .unwrap();

        {
            let recursive_builder = RecursiveBuilder::new(
                // context,
                //  &module,
                &builder,
                f32_type,
                &abs_fn,
                &copysign_fn,
                &sqrt_fn,
                &min_fn,
                &max_fn,
                &x1,
                &y1,
                &x2,
                &y2,
            );
            let return_value = recursive_builder.build(&heuristic.root);
            let _ = builder.build_return(Some(&return_value));
        }

        Jit {
            // context,
            // module,
            function: unsafe { execution_engine.get_function("execute") }.unwrap(),
        }
    }

    pub fn execute(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        unsafe { self.function.call(x1, y1, x2, y2) }
    }
}

struct RecursiveBuilder<'a> {
    // context: &'a Context,
    // module: &'a Module<'a>,
    builder: &'a Builder<'a>,
    f32_type: FloatType<'a>,
    abs_fn: &'a FunctionValue<'a>,
    copysign_fn: &'a FunctionValue<'a>,
    sqrt_fn: &'a FunctionValue<'a>,
    min_fn: &'a FunctionValue<'a>,
    max_fn: &'a FunctionValue<'a>,
    x1: &'a FloatValue<'a>,
    y1: &'a FloatValue<'a>,
    x2: &'a FloatValue<'a>,
    y2: &'a FloatValue<'a>,
}

impl<'a> RecursiveBuilder<'a> {
    pub fn new(
        // context: &'a Context,
        // module: &'a Module<'a>,
        builder: &'a Builder<'a>,
        f32_type: FloatType<'a>,
        abs_fn: &'a FunctionValue<'a>,
        copysign_fn: &'a FunctionValue<'a>,
        sqrt_fn: &'a FunctionValue<'a>,
        min_fn: &'a FunctionValue<'a>,
        max_fn: &'a FunctionValue<'a>,
        x1: &'a FloatValue<'a>,
        y1: &'a FloatValue<'a>,
        x2: &'a FloatValue<'a>,
        y2: &'a FloatValue<'a>,
    ) -> Self {
        RecursiveBuilder {
            // context,
            // module,
            builder,
            f32_type,
            abs_fn,
            copysign_fn,
            sqrt_fn,
            min_fn,
            max_fn,
            x1,
            y1,
            x2,
            y2,
        }
    }

    pub fn build(&self, node: &HeuristicNode) -> FloatValue {
        match node {
            HeuristicNode::Number(num) => self.f32_type.const_float(*num as f64),
            HeuristicNode::Terminal(rule) => self.build_terminal(*rule),
            HeuristicNode::Unary(rule, h) => self.build_unary(*rule, h),
            HeuristicNode::Binary(rule, h1, h2) => self.build_binary(*rule, h1, h2),
        }
    }

    fn build_terminal(&self, rule: Rule) -> FloatValue {
        match rule {
            Rule::x1 => *self.x1,
            Rule::y1 => *self.y1,
            Rule::x2 => *self.x2,
            Rule::y2 => *self.y2,
            Rule::deltaX => {
                let deltax = self
                    .builder
                    .build_float_sub(*self.x2, *self.x1, "deltaX")
                    .unwrap();

                let abs = self
                    .builder
                    .build_call(*self.abs_fn, &[deltax.into()], "abs")
                    .unwrap();
                abs.try_as_basic_value().left().unwrap().into_float_value()
            }
            Rule::deltaY => {
                let deltay = self
                    .builder
                    .build_float_sub(*self.y2, *self.y1, "deltaY")
                    .unwrap();

                let abs = self
                    .builder
                    .build_call(*self.abs_fn, &[deltay.into()], "abs")
                    .unwrap();
                abs.try_as_basic_value().left().unwrap().into_float_value()
            }
            _ => {
                unreachable!("{:?}", rule);
            }
        }
    }

    fn build_unary(&self, rule: Rule, h: &HeuristicNode) -> FloatValue {
        let result = self.build(h);
        match rule {
            Rule::neg => self.builder.build_float_neg(result, "neg").unwrap(),
            Rule::abs => {
                let abs = self
                    .builder
                    .build_call(*self.abs_fn, &[result.into()], "abs")
                    .unwrap();
                abs.try_as_basic_value().left().unwrap().into_float_value()
            }
            Rule::sqrt => {
                let abs = self
                    .builder
                    .build_call(*self.abs_fn, &[result.into()], "abs")
                    .unwrap();
                let abs = abs.try_as_basic_value().left().unwrap().into_float_value();

                let sqrt = self
                    .builder
                    .build_call(*self.sqrt_fn, &[abs.into()], "sqrt")
                    .unwrap();
                let sqrt = sqrt.try_as_basic_value().left().unwrap().into_float_value();

                let copysign = self
                    .builder
                    .build_call(*self.copysign_fn, &[sqrt.into(), result.into()], "copysign")
                    .unwrap();
                copysign
                    .try_as_basic_value()
                    .left()
                    .unwrap()
                    .into_float_value()
            }
            Rule::sqr => self.builder.build_float_mul(result, result, "sqr").unwrap(),
            _ => {
                unreachable!("{:?}", rule);
            }
        }
    }

    fn build_binary(&self, rule: Rule, h1: &HeuristicNode, h2: &HeuristicNode) -> FloatValue {
        let result1 = self.build(h1);
        let result2 = self.build(h2);
        match rule {
            Rule::plus => self
                .builder
                .build_float_add(result1, result2, "plus")
                .unwrap(),
            Rule::minus => self
                .builder
                .build_float_sub(result1, result2, "minus")
                .unwrap(),
            Rule::mul => self
                .builder
                .build_float_mul(result1, result2, "mul")
                .unwrap(),
            Rule::div => self
                .builder
                .build_float_div(result1, result2, "div")
                .unwrap(),
            Rule::max => {
                let max = self
                    .builder
                    .build_call(*self.max_fn, &[result1.into(), result2.into()], "max")
                    .unwrap();
                max.try_as_basic_value().left().unwrap().into_float_value()
            }
            Rule::min => {
                let min = self
                    .builder
                    .build_call(*self.min_fn, &[result1.into(), result2.into()], "min")
                    .unwrap();
                min.try_as_basic_value().left().unwrap().into_float_value()
            }
            _ => {
                unreachable!("{:?}", rule);
            }
        }
    }
}
