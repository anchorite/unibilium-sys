#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::fmt;

impl fmt::Display for unibi_boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match *self {
            unibi_boolean::unibi_auto_left_margin => "auto_left_margin",
            unibi_boolean::unibi_auto_right_margin => "auto_right_margin",
            unibi_boolean::unibi_no_esc_ctlc => "no_esc_ctlc",
            unibi_boolean::unibi_ceol_standout_glitch => "ceol_standout_glitch",
            unibi_boolean::unibi_eat_newline_glitch => "eat_newline_glitch",
            unibi_boolean::unibi_erase_overstrike => "erase_overstrike",
            unibi_boolean::unibi_generic_type => "generic_type",
            unibi_boolean::unibi_hard_copy => "hard_copy",
            unibi_boolean::unibi_has_meta_key => "has_meta_key",
            unibi_boolean::unibi_has_status_line => "has_status_line",
            unibi_boolean::unibi_insert_null_glitch => "insert_null_glitch",
            unibi_boolean::unibi_memory_above => "memory_above",
            unibi_boolean::unibi_memory_below => "memory_below",
            unibi_boolean::unibi_move_insert_mode => "move_insert_mode",
            unibi_boolean::unibi_move_standout_mode => "move_standout_mode",
            unibi_boolean::unibi_over_strike => "over_strike",
            unibi_boolean::unibi_status_line_esc_ok => "status_line_esc_ok",
            unibi_boolean::unibi_dest_tabs_magic_smso => "dest_tabs_magic_smso",
            unibi_boolean::unibi_tilde_glitch => "tilde_glitch",
            unibi_boolean::unibi_transparent_underline => "transparent_underline",
            unibi_boolean::unibi_xon_xoff => "xon_xoff",
            unibi_boolean::unibi_needs_xon_xoff => "needs_xon_xoff",
            unibi_boolean::unibi_prtr_silent => "prtr_silent",
            unibi_boolean::unibi_hard_cursor => "hard_cursor",
            unibi_boolean::unibi_non_rev_rmcup => "non_rev_rmcup",
            unibi_boolean::unibi_no_pad_char => "no_pad_char",
            unibi_boolean::unibi_non_dest_scroll_region => "non_dest_scroll_region",
            unibi_boolean::unibi_can_change => "can_change",
            unibi_boolean::unibi_back_color_erase => "back_color_erase",
            unibi_boolean::unibi_hue_lightness_saturation => "hue_lightness_saturation",
            unibi_boolean::unibi_col_addr_glitch => "col_addr_glitch",
            unibi_boolean::unibi_cr_cancels_micro_mode => "cr_cancels_micro_mode",
            unibi_boolean::unibi_has_print_wheel => "has_print_wheel",
            unibi_boolean::unibi_row_addr_glitch => "row_addr_glitch",
            unibi_boolean::unibi_semi_auto_right_margin => "semi_auto_right_margin",
            unibi_boolean::unibi_cpi_changes_res => "cpi_changes_res",
            unibi_boolean::unibi_lpi_changes_res => "lpi_changes_res",
            unibi_boolean::unibi_backspaces_with_bs => "backspaces_with_bs",
            unibi_boolean::unibi_crt_no_scrolling => "crt_no_scrolling",
            unibi_boolean::unibi_no_correctly_working_cr => "no_correctly_working_cr",
            unibi_boolean::unibi_gnu_has_meta_key => "gnu_has_meta_key",
            unibi_boolean::unibi_linefeed_is_newline => "linefeed_is_newline",
            unibi_boolean::unibi_has_hardware_tabs => "has_hardware_tabs",
            unibi_boolean::unibi_return_does_clr_eol => "return_does_clr_eol",
            unibi_boolean::unibi_boolean_end_ => "boolean_end_",
            _ => return write!(f, "unknown boolean({})", self.0),
        };

        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean_display() {
        assert_eq!(
            format!("{}", unibi_boolean::unibi_return_does_clr_eol),
            "return_does_clr_eol"
        );

        assert_eq!(
            format!("{}", unibi_boolean(unibi_boolean::unibi_boolean_end_.0 + 1)),
            format!(
                "unknown boolean({})",
                unibi_boolean::unibi_boolean_end_.0 + 1
            )
        );
    }
}
