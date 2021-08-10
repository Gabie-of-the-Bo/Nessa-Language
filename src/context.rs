use crate::types::*;
use crate::object::*;
use crate::operations::*;

/*
                                                  ╒══════════════════╕
    ============================================= │  IMPLEMENTATION  │ =============================================
                                                  ╘══════════════════╛
*/

#[derive(Default, Clone)]
pub struct NessaContext {
    pub type_templates: Vec<TypeTemplate>, 

    pub unary_ops: Vec<UnaryOperator>,
    pub binary_ops: Vec<BinaryOperator>,
    pub nary_ops: Vec<NaryOperator>
}

impl NessaContext {

    /*
        ╒════════════════════════════╕
        │ Type template manipulation │
        ╘════════════════════════════╛
    */

    pub fn define_type(&mut self, representation: String, params: Vec<String>) -> Result<(), String> {
        for t in &self.type_templates {
            if t.name == representation {
                return Err(format!("Type \"{}\" is already defined", representation))
            }
        }

        self.type_templates.push(TypeTemplate {
            id: self.type_templates.len(),
            name: representation,
            params: params
        });

        return Ok(());
    }

    /*
        ╒═════════════════════════════╕
        │ Unary operator manipulation │
        ╘═════════════════════════════╛
    */

    pub fn define_unary_operator(&mut self, representation: String) -> Result<(), String> {
        for o in &self.unary_ops {
            if o.representation == representation {
                return Err(format!("Unary operator \"{}\" is already defined", representation))
            }
        }

        self.unary_ops.push(UnaryOperator {
            id: self.unary_ops.len(),
            representation: representation,
            operations: vec!()
        });

        return Ok(());
    }

    pub fn get_unary_operations(&self, id: usize, a: Type) -> Vec<&(Type, UnaryFunction)> {        
        return self.unary_ops[id].operations.iter().filter(|(t, _)| a.bindable_to(&t)).collect::<Vec<_>>();
    }

    pub fn def_unary_operation(&mut self, id: usize, a: Type, f: UnaryFunction) -> Result<(), String> {
        let op = &self.unary_ops[id];

        for (t, _) in &op.operations{ // Check subsumption
            if a.bindable_to(&t) {
                return Err(format!("Unary operation {}{} is subsumed by {}{}, so it cannot be defined", 
                                    op.representation, a.get_name(self), op.representation, t.get_name(self)));
            }

            if t.bindable_to(&a) {
                return Err(format!("Unary operation {}{} subsumes {}{}, so it cannot be defined", 
                                    op.representation, a.get_name(self), op.representation, t.get_name(self)));
            }
        }

        self.unary_ops[id].operations.push((a, f));

        return Ok(());
    }

    /*
        ╒══════════════════════════════╕
        │ Binary operator manipulation │
        ╘══════════════════════════════╛
    */

    pub fn define_binary_operator(&mut self, representation: String) -> Result<(), String> {
        for o in &self.binary_ops {
            if o.representation == representation {
                return Err(format!("Binary operator \"{}\" is already defined", representation))
            }
        }

        self.binary_ops.push(BinaryOperator {
            id: self.binary_ops.len(),
            representation: representation,
            operations: vec!()
        });

        return Ok(());
    }

    pub fn get_binary_operations(&self, id: usize, a: Type, b: Type) -> Vec<&(Type, BinaryFunction)> {
        let args_type = Type::And(vec!(a, b));

        return self.binary_ops[id].operations.iter().filter(|(t, _)| args_type.bindable_to(&t)).collect::<Vec<_>>();
    }

    pub fn def_binary_operation(&mut self, id: usize, a: Type, b: Type, f: BinaryFunction) -> Result<(), String> {
        let and = Type::And(vec!(a.clone(), b.clone()));
        let op = &self.binary_ops[id];

        for (t, _) in &op.operations{ // Check subsumption
            if let Type::And(v) = t {
                if and.bindable_to(&t) {
                    return Err(format!("Binary operation {} {} {} is subsumed by {} {} {}, so it cannot be defined", 
                                        a.get_name(self), op.representation, b.get_name(self), 
                                        v[0].get_name(self), op.representation, v[1].get_name(self)));
                }

                if t.bindable_to(&and) {
                    return Err(format!("Binary operation {} {} {} subsumes {} {} {}, so it cannot be defined", 
                                        a.get_name(self), op.representation, b.get_name(self), 
                                        v[0].get_name(self), op.representation, v[1].get_name(self)));
                }
            }
        }

        self.binary_ops[id].operations.push((and, f));

        return Ok(());
    }

    /*
        ╒═════════════════════════════╕
        │ N-ary operator manipulation │
        ╘═════════════════════════════╛
    */

    pub fn define_nary_operator(&mut self, open_rep: String, close_rep: String) -> Result<(), String> {
        for o in &self.nary_ops {
            if o.open_rep == open_rep || o.close_rep == close_rep {
                return Err(format!("N-ary operator \"{}{}\" has a syntax overlap with \"{}{}\", so it cannot be defined", 
                                    open_rep, close_rep, o.open_rep, o.close_rep))
            }
        }

        self.nary_ops.push(NaryOperator {
            id: self.nary_ops.len(),
            open_rep: open_rep,
            close_rep: close_rep,
            operations: vec!()
        });

        return Ok(());
    }

    pub fn get_nary_operation(&self, id: usize, from: &Object, args: &[&Object]) -> Option<&NaryFunction> {
        let mut subtypes = vec!(from.get_type());
        subtypes.extend(args.iter().map(|i| i.get_type()));

        let args_type = Type::And(subtypes);

        for (t, op) in &self.nary_ops[id].operations{ // Check subsumption
            if args_type.bindable_to(&t) {
                return Some(op);
            }
        }
        
        return None;
    }

    pub fn def_nary_operation(&mut self, id: usize, from: Type, args: &[Type], f: NaryFunction) -> Result<(), String> {
        let mut subtypes = vec!(from.clone());
        subtypes.extend(args.into_iter().cloned());

        let and = Type::And(subtypes);
        let op = &self.nary_ops[id];

        for (t, _) in &op.operations{ // Check subsumption
            if let Type::And(v) = t {
                if and.bindable_to(&t) {
                    return Err(format!("Binary operation {}{}{}{} is subsumed by {}{}{}{}, so it cannot be defined", 
                                        from.get_name(self), op.open_rep, args.iter().map(|i| i.get_name(self)).collect::<Vec<_>>().join(", "), op.close_rep, 
                                        v[0].get_name(self), op.open_rep, v[1..].iter().map(|i| i.get_name(self)).collect::<Vec<_>>().join(", "), op.close_rep));
                }

                if t.bindable_to(&and) {
                    return Err(format!("Binary operation {}{}{}{} subsumes {}{}{}{}, so it cannot be defined", 
                                        from.get_name(self), op.open_rep, args.iter().map(|i| i.get_name(self)).collect::<Vec<_>>().join(", "), op.close_rep, 
                                        v[0].get_name(self), op.open_rep, v[1..].iter().map(|i| i.get_name(self)).collect::<Vec<_>>().join(", "), op.close_rep));
                }
            }
        }

        self.nary_ops[id].operations.push((and, f));

        return Ok(());
    }
}

/*
                                                  ╒═════════╕
    ============================================= │  TESTS  │ =============================================
                                                  ╘═════════╛
*/

#[cfg(test)]
mod tests {
    use crate::types::Type;
    use crate::context::*;

    #[test]
    fn operation_subsumption() {
        let mut ctx = standard_ctx();

        let def_1 = ctx.def_unary_operation(0, Type::Basic(1), |a| { a.clone() });
        let def_2 = ctx.def_unary_operation(0, Type::Basic(0), |a| { a.clone() });
        let def_3 = ctx.def_unary_operation(0, Type::Wildcard, |a| { a.clone() });

        assert!(def_1.is_ok());
        assert!(def_2.is_err());
        assert!(def_3.is_err());

        let def_1 = ctx.def_binary_operation(0, Type::Basic(0), Type::Basic(1), |a, _| { a.clone() });
        let def_2 = ctx.def_binary_operation(0, Type::Basic(1), Type::Basic(1), |a, _| { a.clone() });
        let def_3 = ctx.def_binary_operation(0, Type::Wildcard, Type::Wildcard, |a, _| { a.clone() });

        assert!(def_1.is_ok());
        assert!(def_2.is_err());
        assert!(def_3.is_err());

        let def_1 = ctx.def_nary_operation(0, Type::Basic(0), &[Type::Basic(0)], |a, _| { a.clone() });
        let def_2 = ctx.def_nary_operation(0, Type::Basic(1), &[Type::Ref(Box::new(Type::Basic(1)))], |a, _| { a.clone() });
        let def_3 = ctx.def_nary_operation(0, Type::Basic(1), &[Type::Basic(1)], |a, _| { a.clone() });
        let def_4 = ctx.def_nary_operation(0, Type::Wildcard, &[Type::Wildcard], |a, _| { a.clone() });

        assert!(def_1.is_ok());
        assert!(def_2.is_ok());
        assert!(def_3.is_err());
        assert!(def_4.is_err());
    }

    #[test]
    fn operator_redefinition() {
        let mut ctx = standard_ctx();

        let def_1 = ctx.define_unary_operator("~".into());
        let def_2 = ctx.define_unary_operator("-".into());

        assert!(def_1.is_ok());
        assert!(def_2.is_err());

        let def_1 = ctx.define_binary_operator("-".into());
        let def_2 = ctx.define_binary_operator("+".into());

        assert!(def_1.is_ok());
        assert!(def_2.is_err());

        let def_1 = ctx.define_nary_operator("[".into(), "]".into());
        let def_2 = ctx.define_nary_operator("(".into(), ")".into());
        let def_3 = ctx.define_nary_operator("{".into(), ")".into());
        let def_4 = ctx.define_nary_operator("(".into(), "}".into());

        assert!(def_1.is_ok());
        assert!(def_2.is_err());
        assert!(def_3.is_err());
        assert!(def_4.is_err());
    }

    #[test]
    fn type_redefinition() {
        let mut ctx = standard_ctx();

        let def_1 = ctx.define_type("Matrix".into(), vec!());
        let def_2 = ctx.define_type("Number".into(), vec!());

        assert!(def_1.is_ok());
        assert!(def_2.is_err());
    }
}

/*
                                                  ╒════════════════╕
    ============================================= │  STANDARD CTX  │ =============================================
                                                  ╘════════════════╛
*/

pub fn standard_ctx() -> NessaContext {
    let mut ctx = NessaContext::default(); 

    standard_types(&mut ctx);

    standard_unary_operations(&mut ctx);
    standard_binary_operations(&mut ctx);
    standard_nary_operations(&mut ctx);

    return ctx;
}