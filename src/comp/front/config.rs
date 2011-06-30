import std::option;
import std::vec;
import ast;
import fold;
import attr;

export strip_unconfigured_items;

// Support conditional compilation by transforming the AST, stripping out
// any items that do not belong in the current configuration
fn strip_unconfigured_items(@ast::crate crate) -> @ast::crate {
    auto cfg = crate.node.config;

    auto precursor = rec(fold_mod = bind fold_mod(cfg, _, _)
                         with *fold::default_ast_fold());

    auto fold = fold::make_fold(precursor);
    auto res = @fold.fold_crate(*crate);
    // FIXME: This is necessary to break a circular reference
    fold::dummy_out(fold);
    ret res;
}

fn filter_item(&ast::crate_cfg cfg,
               &@ast::item item) -> option::t[@ast::item] {
    if (in_cfg(cfg, item)) {
        option::some(item)
    } else {
        option::none
    }
}

fn fold_mod(&ast::crate_cfg cfg, &ast::_mod m,
              fold::ast_fold fld) -> ast::_mod {
    auto filter = bind filter_item(cfg, _);
    auto filtered_items = vec::filter_map(filter, m.items);
    ret rec(view_items=vec::map(fld.fold_view_item, m.view_items),
            items=vec::map(fld.fold_item, filtered_items));
}

// Determine if an item should be translated in the current crate
// configuration based on the item's attributes
fn in_cfg(&ast::crate_cfg cfg, &@ast::item item) -> bool {

    auto item_cfg_attrs = attr::find_attrs_by_name(item.attrs, "cfg");

    auto item_has_cfg_attrs = vec::len(item_cfg_attrs) > 0u;
    if (!item_has_cfg_attrs) { ret true; }

    auto item_cfg_metas = attr::attr_metas(item_cfg_attrs);

    for (@ast::meta_item cfg_mi in item_cfg_metas) {
        if (attr::contains(cfg, cfg_mi)) {
            ret true;
        }
    }

    ret false;
}


// Local Variables:
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// compile-command: "make -k -C $RBUILD 2>&1 | sed -e 's/\\/x\\//x:\\//g'";
// End:
