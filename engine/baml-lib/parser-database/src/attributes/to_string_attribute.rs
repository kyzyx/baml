use baml_types::Constraint;
use internal_baml_diagnostics::{DatamodelError, Span};

use itertools::Itertools;

use crate::{context::Context, types::Attributes};

use super::alias::visit_alias_attribute;
use super::constraint::visit_constraint_attributes;
use super::description::visit_description_attribute;
pub(super) fn visit(ctx: &mut Context<'_>, span: &Span, as_block: bool) -> Option<Attributes> {
    let mut modified = false;

    let mut attributes = Attributes::default();
    // @alias or @@alias
    if ctx.visit_optional_single_attr("alias") {
        visit_alias_attribute(&mut attributes, ctx);
        modified = true;
        ctx.validate_visited_arguments();
    }

    if ctx.visit_optional_single_attr("description") {
        visit_description_attribute(&mut attributes, ctx);
        modified = true;
        ctx.validate_visited_arguments();
    }

    if ctx.visit_optional_single_attr("skip") {
        attributes.set_skip();
        modified = true;
        ctx.validate_visited_arguments();
    }

    if let Some((attribute_name, span)) = ctx.visit_repeated_attr_from_names(&["assert", "check"]) {
        visit_constraint_attributes(attribute_name, span, &mut attributes, ctx);
        modified = true;
        ctx.validate_visited_arguments();
    }

    if as_block && ctx.visit_optional_single_attr("dynamic") {
        attributes.set_dynamic_type();
        modified = true;
        ctx.validate_visited_arguments();
    }

    if modified {
        Some(attributes)
    } else {
        None
    }
}
