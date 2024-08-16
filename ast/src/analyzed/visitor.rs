use crate::parsed::visitor::VisitOrder;

use super::*;

impl<T> ExpressionVisitable<AlgebraicExpression<T>> for AlgebraicExpression<T> {
    fn visit_expressions_mut<F, B>(&mut self, f: &mut F, o: VisitOrder) -> ControlFlow<B>
    where
        F: FnMut(&mut AlgebraicExpression<T>) -> ControlFlow<B>,
    {
        if o == VisitOrder::Pre {
            if let ControlFlow::Break(b) = f(self)? {
                return ControlFlow::Break(b);
            }
        }

        match self {
            AlgebraicExpression::Reference(_)
            | AlgebraicExpression::PublicReference(_)
            | AlgebraicExpression::Number(_) => {},
            AlgebraicExpression::BinaryOperation(left, _, right) => {
                if let ControlFlow::Break(b) = left.visit_expressions_mut(f, o)? {
                    return ControlFlow::Break(b);
                }
                if let ControlFlow::Break(b) = right.visit_expressions_mut(f, o)? {
                    return ControlFlow::Break(b);
                }
            },
            AlgebraicExpression::UnaryOperation(_, e) => {
                if let ControlFlow::Break(b) = e.visit_expressions_mut(f, o)? {
                    return ControlFlow::Break(b);
                }
            },
        }

        if o == VisitOrder::Post {
            if let ControlFlow::Break(b) = f(self)? {
                return ControlFlow::Break(b);
            }
        }
        ControlFlow::Continue(())
    }

    fn visit_expressions<F, B>(&self, f: &mut F, o: VisitOrder) -> ControlFlow<B>
    where
        F: FnMut(&AlgebraicExpression<T>) -> ControlFlow<B>,
    {
        if o == VisitOrder::Pre {
            if let ControlFlow::Break(b) = f(self)? {
                return ControlFlow::Break(b);
            }
        }

        match self {
            AlgebraicExpression::Reference(_)
            | AlgebraicExpression::PublicReference(_)
            | AlgebraicExpression::Number(_) => {},
            AlgebraicExpression::BinaryOperation(left, _, right) => {
                if let ControlFlow::Break(b) = left.visit_expressions(f, o)? {
                    return ControlFlow::Break(b);
                }
                if let ControlFlow::Break(b) = right.visit_expressions(f, o)? {
                    return ControlFlow::Break(b);
                }
            },
            AlgebraicExpression::UnaryOperation(_, e) => {
                if let ControlFlow::Break(b) = e.visit_expressions(f, o)? {
                    return ControlFlow::Break(b);
                }
            },
        }

        if o == VisitOrder::Post {
            if let ControlFlow::Break(b) = f(self)? {
                return ControlFlow::Break(b);
            }
        }
        ControlFlow::Continue(())
    }
}

impl<Expr: ExpressionVisitable<Expr>> ExpressionVisitable<Expr> for Identity<Expr> {
    fn visit_expressions_mut<F, B>(&mut self, f: &mut F, o: VisitOrder) -> ControlFlow<B>
    where
        F: FnMut(&mut Expr) -> ControlFlow<B>,
    {
        let mut iter = self.left.selector.as_mut()
            .into_iter()
            .chain(self.left.expressions.iter_mut())
            .chain(self.right.selector.as_mut())
            .chain(self.right.expressions.iter_mut());

        while let Some(item) = iter.next() {
            if let ControlFlow::Break(b) = item.visit_expressions_mut(f, o)? {
                return ControlFlow::Break(b);
            }
        }
        ControlFlow::Continue(())
    }

    fn visit_expressions<F, B>(&self, f: &mut F, o: VisitOrder) -> ControlFlow<B>
    where
        F: FnMut(&Expr) -> ControlFlow<B>,
    {
        let mut iter = self.left.selector.as_ref()
            .into_iter()
            .chain(self.left.expressions.iter())
            .chain(self.right.selector.iter())
            .chain(self.right.expressions.iter());

        while let Some(item) = iter.next() {
            if let ControlFlow::Break(b) = item.visit_expressions(f, o)? {
                return ControlFlow::Break(b);
            }
        }
        ControlFlow::Continue(())
    }
}

impl<T> ExpressionVisitable<Expression<T>> for FunctionValueDefinition<T> {
    fn visit_expressions_mut<F, B>(&mut self, f: &mut F, o: VisitOrder) -> ControlFlow<B>
    where
        F: FnMut(&mut Expression<T>) -> ControlFlow<B>,
    {
        match self {
            FunctionValueDefinition::Query(e) | FunctionValueDefinition::Expression(e) => {
                e.visit_expressions_mut(f, o)
            }
            FunctionValueDefinition::Array(array) => {
                let mut iter = array.iter_mut()
                    .flat_map(|a| a.pattern.iter_mut());

                while let Some(item) = iter.next() {
                    if let ControlFlow::Break(b) = item.visit_expressions_mut(f, o)? {
                        return ControlFlow::Break(b);
                    }
                }
                ControlFlow::Continue(())
            }
            FunctionValueDefinition::Number(_) => ControlFlow::Continue(()),
        }
    }

    fn visit_expressions<F, B>(&self, f: &mut F, o: VisitOrder) -> ControlFlow<B>
    where
        F: FnMut(&Expression<T>) -> ControlFlow<B>,
    {
        match self {
            FunctionValueDefinition::Query(e) | FunctionValueDefinition::Expression(e) => {
                e.visit_expressions(f, o)
            }
            FunctionValueDefinition::Array(array) => {
                let mut iter = array.iter()
                    .flat_map(|a| a.pattern().iter());

                while let Some(item) = iter.next() {
                    if let ControlFlow::Break(b) = item.visit_expressions(f, o)? {
                        return ControlFlow::Break(b);
                    }
                }
                ControlFlow::Continue(())
            }
            FunctionValueDefinition::Number(_) => ControlFlow::Continue(()),
        }
    }
}
