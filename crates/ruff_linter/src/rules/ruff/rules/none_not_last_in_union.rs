use crate::checkers::ast::Checker;
use crate::rules::ruff::rules::UnionLike;
use itertools::Itertools;
use ruff_diagnostics::{AlwaysFixableViolation, Diagnostic, Edit, Fix};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::{Expr, ExprBinOp, ExprSubscript, Operator};
use ruff_python_semantic::analyze::typing::traverse_union;
use ruff_python_semantic::SemanticModel;
use ruff_text_size::{Ranged, TextRange};

/// ## What it does
/// Checks for unions where `None` is not the last member.
///
/// ## Why is this bad?
/// When an union has `None` as a member, it usually signifies a missing value,
/// a last resort that should not be focused on as much as the other members.
/// It is thus recommended to place `None` at the very last of the union.
///
/// ## Example
/// ```python
/// None | int
/// Union[str, None, bytes]
/// ```
///
/// Use instead:
/// ```python
/// int | None
/// Union[str, bytes, None]
/// ```
///
/// ## Fix safety
/// The fix is marked as unsafe by default,
/// since it attempts to flatted nested unions
/// and might remove comments in the process.
#[violation]
pub struct NoneNotLastInUnion;

impl AlwaysFixableViolation for NoneNotLastInUnion {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`None` is not the last member of a union".to_string()
    }

    fn fix_title(&self) -> String {
        "Move `None` to the back".to_string()
    }
}

/// RUF036
pub(crate) fn none_not_last_in_union<'a>(checker: &mut Checker, expr: &'a Expr) {
    let semantic = checker.semantic();
    let mut members = vec![];

    let mut collect_members = |expr: &'a Expr, _parent: &'a Expr| {
        if union_kind(semantic, expr).is_none() {
            members.push(expr);
        }
    };

    traverse_union(&mut collect_members, semantic, expr);

    if members.len() < 2 || members.last().unwrap().is_none_literal_expr() {
        return;
    }

    let Some((none_index, none)) = members
        .iter()
        .find_position(|expr| expr.is_none_literal_expr())
    else {
        return;
    };
    let none_range = none.range();

    if none_index + 1 == members.len() {
        return;
    }

    let (members_range, sorted_members) = move_none_to_back_preserving_order(members, none_index);
    let Some(fix) = sort_union_fix(checker, expr, sorted_members, members_range) else {
        return;
    };

    let diagnostic = Diagnostic::new(NoneNotLastInUnion, none_range);

    checker.diagnostics.push(diagnostic.with_fix(fix));
}

fn union_kind(semantic: &SemanticModel, expr: &Expr) -> Option<UnionLike> {
    match expr {
        Expr::BinOp(ExprBinOp {
            op: Operator::BitOr,
            ..
        }) => Some(UnionLike::PEP604),

        Expr::Subscript(ExprSubscript { value, .. })
            if semantic.match_typing_expr(value, "Union") =>
        {
            Some(UnionLike::TypingUnion)
        }

        _ => None,
    }
}

/// Returns a fix that flattens the entire union
/// And move the `None` member found to the back.
///
/// This fix might result in multiple `None`s.
/// That is left for
fn sort_union_fix(
    checker: &Checker,
    expr: &Expr,
    sorted_members: Vec<&Expr>,
    members_range: TextRange,
) -> Option<Fix> {
    let (locator, semantic) = (checker.locator(), checker.semantic());
    let union_kind = union_kind(semantic, expr)?;

    let mut member_text = sorted_members
        .iter()
        .map(|expr| locator.slice(expr.range()));

    let edit = match union_kind {
        UnionLike::TypingUnion => {
            let new_content = member_text.join(", ");

            Edit::range_replacement(new_content, members_range)
        }
        UnionLike::PEP604 => {
            let union_range = expr.range();
            let new_content = member_text.join(" | ");

            Edit::range_replacement(new_content, union_range)
        }
    };

    Some(Fix::unsafe_edit(edit))
}

#[inline]
fn move_none_to_back_preserving_order(
    mut members: Vec<&Expr>,
    index: usize,
) -> (TextRange, Vec<&Expr>) {
    let (first, last) = (members.first().unwrap(), members.last().unwrap());
    let members_range = TextRange::new(first.start(), last.end());

    let none = members.remove(index);
    members.push(none);

    (members_range, members)
}
