use crate::rules::*;

impl Rule {
	pub fn to_css_syntax(&self) -> String {
		let property = match self.property {
			Property::AlignContent => "align-content",
			Property::AlignItems => "align-items",
			Property::AlignSelf => "align-self",
			Property::All => "all",
			Property::Animation => "animation",
			Property::AnimationDelay => "animation-delay",
			Property::AnimationDirection => "animation-direction",
			Property::AnimationDuration => "animation-duration",
			Property::AnimationFillMode => "animation-fill-mode",
			Property::AnimationIterationCount => "animation-iteration-count",
			Property::AnimationName => "animation-name",
			Property::AnimationPlayState => "animation-play-state",
			Property::AnimationTimingFunction => "animation-timing-function",
			Property::Azimuth => "azimuth",
			Property::Background => "background",
			Property::BackgroundAttachment => "background-attachment",
			Property::BackgroundBlendMode => "background-blend-mode",
			Property::BackgroundClip => "background-clip",
			Property::BackgroundColor => "background-color",
			Property::BackgroundImage => "background-image",
			Property::BackgroundOrigin => "background-origin",
			Property::BackgroundPosition => "background-position",
			Property::BackgroundRepeat => "background-repeat",
			Property::BackgroundSize => "background-size",
			Property::Border => "border",
			Property::BorderBottom => "border-bottom",
			Property::BorderBottomColor => "border-bottom-color",
			Property::BorderBottomLeftRadius => "border-bottom-left-radius",
			Property::BorderBottomRightRadius => "border-bottom-right-radius",
			Property::BorderBottomStyle => "border-bottom-style",
			Property::BorderBottomWidth => "border-bottom-width",
			Property::BorderCollapse => "border-collapse",
			Property::BorderColor => "border-color",
			Property::BorderImage => "border-image",
			Property::BorderImageOutset => "border-image-outset",
			Property::BorderImageRepeat => "border-image-repeat",
			Property::BorderImageSlice => "border-image-slice",
			Property::BorderImageSource => "border-image-source",
			Property::BorderImageWidth => "border-image-width",
			Property::BorderLeft => "border-left",
			Property::BorderLeftColor => "border-left-color",
			Property::BorderLeftStyle => "border-left-style",
			Property::BorderLeftWidth => "border-left-width",
			Property::BorderRadius => "border-radius",
			Property::BorderRight => "border-right",
			Property::BorderRightColor => "border-right-color",
			Property::BorderRightStyle => "border-right-style",
			Property::BorderRightWidth => "border-right-width",
			Property::BorderSpacing => "border-spacing",
			Property::BorderStyle => "border-style",
			Property::BorderTop => "border-top",
			Property::BorderTopColor => "border-top-color",
			Property::BorderTopLeftRadius => "border-top-left-radius",
			Property::BorderTopRightRadius => "border-top-right-radius",
			Property::BorderTopStyle => "border-top-style",
			Property::BorderTopWidth => "border-top-width",
			Property::BorderWidth => "border-width",
			Property::Bottom => "bottom",
			Property::BoxDecorationBreak => "box-decoration-break",
			Property::BoxShadow => "box-shadow",
			Property::BoxSizing => "box-sizing",
			Property::BreakAfter => "break-after",
			Property::BreakBefore => "break-before",
			Property::BreakInside => "break-inside",
			Property::CaptionSide => "caption-side",
			Property::CaretColor => "caret-color",
			Property::Clear => "clear",
			Property::Clip => "clip",
			Property::ClipPath => "clip-path",
			Property::ClipRule => "clip-rule",
			Property::Color => "color",
			Property::ColorInterpolationFilters => "color-interpolation-filters",
			Property::ColumnCount => "column-count",
			Property::ColumnFill => "column-fill",
			Property::ColumnGap => "column-gap",
			Property::ColumnRule => "column-rule",
			Property::ColumnRuleColor => "column-rule-color",
			Property::ColumnRuleStyle => "column-rule-style",
			Property::ColumnRuleWidth => "column-rule-width",
			Property::Columns => "columns",
			Property::ColumnSpan => "column-span",
			Property::ColumnWidth => "column-width",
			Property::Contain => "contain",
			Property::Content => "content",
			Property::CounterIncrement => "counter-increment",
			Property::CounterReset => "counter-reset",
			Property::Cue => "cue",
			Property::CueAfter => "cue-after",
			Property::CueBefore => "cue-before",
			Property::Cursor => "cursor",
			Property::Direction => "direction",
			Property::Display => "display",
			Property::Elevation => "elevation",
			Property::EmptyCells => "empty-cells",
			Property::Filter => "filter",
			Property::Flex => "flex",
			Property::FlexBasis => "flex-basis",
			Property::FlexDirection => "flex-direction",
			Property::FlexFlow => "flex-flow",
			Property::FlexGrow => "flex-grow",
			Property::FlexShrink => "flex-shrink",
			Property::FlexWrap => "flex-wrap",
			Property::Float => "float",
			Property::FloodColor => "flood-color",
			Property::FloodOpacity => "flood-opacity",
			Property::Font => "font",
			Property::FontFamily => "font-family",
			Property::FontFeatureSettings => "font-feature-settings",
			Property::FontKerning => "font-kerning",
			Property::FontSize => "font-size",
			Property::FontSizeAdjust => "font-size-adjust",
			Property::FontStretch => "font-stretch",
			Property::FontStyle => "font-style",
			Property::FontSynthesis => "font-synthesis",
			Property::FontVariant => "font-variant",
			Property::FontVariantCaps => "font-variant-caps",
			Property::FontVariantEastAsian => "font-variant-east-asian",
			Property::FontVariantLigatures => "font-variant-ligatures",
			Property::FontVariantNumeric => "font-variant-numeric",
			Property::FontVariantPosition => "font-variant-position",
			Property::FontWeight => "font-weight",
			Property::Gap => "gap",
			Property::Globalcompositeoperation => "globalcompositeoperation",
			Property::GlyphOrientationVertical => "glyph-orientation-vertical",
			Property::Grid => "grid",
			Property::GridArea => "grid-area",
			Property::GridAutoColumns => "grid-auto-columns",
			Property::GridAutoFlow => "grid-auto-flow",
			Property::GridAutoRows => "grid-auto-rows",
			Property::GridColumn => "grid-column",
			Property::GridColumnEnd => "grid-column-end",
			Property::GridColumnGap => "grid-column-gap",
			Property::GridColumnStart => "grid-column-start",
			Property::GridGap => "grid-gap",
			Property::GridRow => "grid-row",
			Property::GridRowEnd => "grid-row-end",
			Property::GridRowGap => "grid-row-gap",
			Property::GridRowStart => "grid-row-start",
			Property::GridTemplate => "grid-template",
			Property::GridTemplateAreas => "grid-template-areas",
			Property::GridTemplateColumns => "grid-template-columns",
			Property::GridTemplateRows => "grid-template-rows",
			Property::HangingPunctuation => "hanging-punctuation",
			Property::Height => "height",
			Property::Hyphens => "hyphens",
			Property::ImageOrientation => "image-orientation",
			Property::ImageRendering => "image-rendering",
			Property::ImageResolution => "image-resolution",
			Property::Isolation => "isolation",
			Property::JustifyContent => "justify-content",
			Property::JustifyItems => "justify-items",
			Property::JustifySelf => "justify-self",
			Property::Left => "left",
			Property::LetterSpacing => "letter-spacing",
			Property::LightingColor => "lighting-color",
			Property::LineBreak => "line-break",
			Property::LineHeight => "line-height",
			Property::ListStyle => "list-style",
			Property::ListStyleImage => "list-style-image",
			Property::ListStylePosition => "list-style-position",
			Property::ListStyleType => "list-style-type",
			Property::Margin => "margin",
			Property::MarginBottom => "margin-bottom",
			Property::MarginLeft => "margin-left",
			Property::MarginRight => "margin-right",
			Property::MarginTop => "margin-top",
			Property::Mask => "mask",
			Property::MaskBorder => "mask-border",
			Property::MaskBorderMode => "mask-border-mode",
			Property::MaskBorderOutset => "mask-border-outset",
			Property::MaskBorderRepeat => "mask-border-repeat",
			Property::MaskBorderSlice => "mask-border-slice",
			Property::MaskBorderSource => "mask-border-source",
			Property::MaskBorderWidth => "mask-border-width",
			Property::MaskClip => "mask-clip",
			Property::MaskComposite => "mask-composite",
			Property::MaskImage => "mask-image",
			Property::MaskMode => "mask-mode",
			Property::MaskOrigin => "mask-origin",
			Property::MaskPosition => "mask-position",
			Property::MaskRepeat => "mask-repeat",
			Property::MaskSize => "mask-size",
			Property::MaskType => "mask-type",
			Property::MaxHeight => "max-height",
			Property::MaxWidth => "max-width",
			Property::MinHeight => "min-height",
			Property::MinWidth => "min-width",
			Property::MixBlendMode => "mix-blend-mode",
			Property::ObjectFit => "object-fit",
			Property::ObjectPosition => "object-position",
			Property::Opacity => "opacity",
			Property::Order => "order",
			Property::Orphans => "orphans",
			Property::Outline => "outline",
			Property::OutlineColor => "outline-color",
			Property::OutlineOffset => "outline-offset",
			Property::OutlineStyle => "outline-style",
			Property::OutlineWidth => "outline-width",
			Property::Overflow => "overflow",
			Property::OverflowWrap => "overflow-wrap",
			Property::Padding => "padding",
			Property::PaddingBottom => "padding-bottom",
			Property::PaddingLeft => "padding-left",
			Property::PaddingRight => "padding-right",
			Property::PaddingTop => "padding-top",
			Property::PageBreakAfter => "page-break-after",
			Property::PageBreakBefore => "page-break-before",
			Property::PageBreakInside => "page-break-inside",
			Property::Pause => "pause",
			Property::PauseAfter => "pause-after",
			Property::PauseBefore => "pause-before",
			Property::Pitch => "pitch",
			Property::PitchRange => "pitch-range",
			Property::PlaceContent => "place-content",
			Property::PlaceItems => "place-items",
			Property::PlaceSelf => "place-self",
			Property::PlayDuring => "play-during",
			Property::Position => "position",
			Property::Quotes => "quotes",
			Property::Resize => "resize",
			Property::Rest => "rest",
			Property::RestAfter => "rest-after",
			Property::RestBefore => "rest-before",
			Property::Richness => "richness",
			Property::Right => "right",
			Property::RowGap => "row-gap",
			Property::ScrollMargin => "scroll-margin",
			Property::ScrollMarginBlock => "scroll-margin-block",
			Property::ScrollMarginBlockEnd => "scroll-margin-block-end",
			Property::ScrollMarginBlockStart => "scroll-margin-block-start",
			Property::ScrollMarginBottom => "scroll-margin-bottom",
			Property::ScrollMarginInline => "scroll-margin-inline",
			Property::ScrollMarginInlineEnd => "scroll-margin-inline-end",
			Property::ScrollMarginInlineStart => "scroll-margin-inline-start",
			Property::ScrollMarginLeft => "scroll-margin-left",
			Property::ScrollMarginRight => "scroll-margin-right",
			Property::ScrollMarginTop => "scroll-margin-top",
			Property::ScrollPadding => "scroll-padding",
			Property::ScrollPaddingBlock => "scroll-padding-block",
			Property::ScrollPaddingBlockEnd => "scroll-padding-block-end",
			Property::ScrollPaddingBlockStart => "scroll-padding-block-start",
			Property::ScrollPaddingBottom => "scroll-padding-bottom",
			Property::ScrollPaddingInline => "scroll-padding-inline",
			Property::ScrollPaddingInlineEnd => "scroll-padding-inline-end",
			Property::ScrollPaddingInlineStart => "scroll-padding-inline-start",
			Property::ScrollPaddingLeft => "scroll-padding-left",
			Property::ScrollPaddingRight => "scroll-padding-right",
			Property::ScrollPaddingTop => "scroll-padding-top",
			Property::ScrollSnapAlign => "scroll-snap-align",
			Property::ScrollSnapStop => "scroll-snap-stop",
			Property::ScrollSnapType => "scroll-snap-type",
			Property::ShapeImageThreshold => "shape-image-threshold",
			Property::ShapeMargin => "shape-margin",
			Property::ShapeOutside => "shape-outside",
			Property::Speak => "speak",
			Property::SpeakAs => "speak-as",
			Property::SpeakHeader => "speak-header",
			Property::SpeakNumeral => "speak-numeral",
			Property::SpeakPunctuation => "speak-punctuation",
			Property::SpeechRate => "speech-rate",
			Property::Stress => "stress",
			Property::TableLayout => "table-layout",
			Property::TabSize => "tab-size",
			Property::TextAlign => "text-align",
			Property::TextAlignAll => "text-align-all",
			Property::TextAlignLast => "text-align-last",
			Property::TextCombineUpright => "text-combine-upright",
			Property::TextDecoration => "text-decoration",
			Property::TextDecorationColor => "text-decoration-color",
			Property::TextDecorationLine => "text-decoration-line",
			Property::TextDecorationStyle => "text-decoration-style",
			Property::TextEmphasis => "text-emphasis",
			Property::TextEmphasisColor => "text-emphasis-color",
			Property::TextEmphasisPosition => "text-emphasis-position",
			Property::TextEmphasisStyle => "text-emphasis-style",
			Property::TextIndent => "text-indent",
			Property::TextJustify => "text-justify",
			Property::TextOrientation => "text-orientation",
			Property::TextOverflow => "text-overflow",
			Property::TextShadow => "text-shadow",
			Property::TextTransform => "text-transform",
			Property::TextUnderlinePosition => "text-underline-position",
			Property::Top => "top",
			Property::Transform => "transform",
			Property::TransformBox => "transform-box",
			Property::TransformOrigin => "transform-origin",
			Property::Transition => "transition",
			Property::TransitionDelay => "transition-delay",
			Property::TransitionDuration => "transition-duration",
			Property::TransitionProperty => "transition-property",
			Property::TransitionTimingFunction => "transition-timing-function",
			Property::UnicodeBidi => "unicode-bidi",
			Property::VerticalAlign => "vertical-align",
			Property::Visibility => "visibility",
			Property::VoiceBalance => "voice-balance",
			Property::VoiceDuration => "voice-duration",
			Property::VoiceFamily => "voice-family",
			Property::VoicePitch => "voice-pitch",
			Property::VoiceRange => "voice-range",
			Property::VoiceRate => "voice-rate",
			Property::VoiceStress => "voice-stress",
			Property::VoiceVolume => "voice-volume",
			Property::Volume => "volume",
			Property::WhiteSpace => "white-space",
			Property::Widows => "widows",
			Property::Width => "width",
			Property::WillChange => "will-change",
			Property::WordBreak => "word-break",
			Property::WordSpacing => "word-spacing",
			Property::WordWrap => "word-wrap",
			Property::WritingMode => "writing-mode",
			Property::ZIndex => "z-index",
		};
		let value = match self.value {
			Value::Add => "add",
			Value::Additive => "additive",
			Value::Alias => "alias",
			Value::All => "all",
			Value::AllowEnd => "allow-end",
			Value::AllScroll => "all-scroll",
			Value::Alpha => "alpha",
			Value::Alphabetic => "alphabetic",
			Value::Alternate => "alternate",
			Value::AlternateReverse => "alternate-reverse",
			Value::Always => "always",
			Value::Anywhere => "anywhere",
			Value::ArabicIndic => "arabic-indic",
			Value::Arithmetic => "arithmetic",
			Value::Armenian => "armenian",
			Value::Atop => "atop",
			Value::Aural => "aural",
			Value::Auto => "auto",
			Value::AutoFill => "auto-fill",
			Value::AutoFit => "auto-fit",
			Value::Avoid => "avoid",
			Value::AvoidColumn => "avoid-column",
			Value::AvoidPage => "avoid-page",
			Value::AvoidRegion => "avoid-region",
			Value::Backgroundalpha => "backgroundalpha",
			Value::Backgroundimage => "backgroundimage",
			Value::Backwards => "backwards",
			Value::Balance => "balance",
			Value::BalanceAll => "balance-all",
			Value::Baseline => "baseline",
			Value::Bengali => "bengali",
			Value::BidiOverride => "bidi-override",
			Value::Blink => "blink",
			Value::Block => "block",
			Value::BorderBox => "border-box",
			Value::Both => "both",
			Value::Bottom => "bottom",
			Value::Braille => "braille",
			Value::BreakAll => "break-all",
			Value::BreakSpaces => "break-spaces",
			Value::BreakWord => "break-word",
			Value::Bullets => "bullets",
			Value::Cambodian => "cambodian",
			Value::Capitalize => "capitalize",
			Value::Cell => "cell",
			Value::Center => "center",
			Value::Ch => "ch",
			Value::Circle => "circle",
			Value::CjkDecimal => "cjk-decimal",
			Value::CjkEarthlyBranch => "cjk-earthly-branch",
			Value::CjkHeavenlyStem => "cjk-heavenly-stem",
			Value::CjkIdeographic => "cjk-ideographic",
			Value::Clip => "clip",
			Value::Clone => "clone",
			Value::CloseQuote => "close-quote",
			Value::ClosestCorner => "closest-corner",
			Value::ClosestSide => "closest-side",
			Value::Cm => "cm",
			Value::Coarse => "coarse",
			Value::Color => "color",
			Value::ColorBurn => "color-burn",
			Value::ColorDodge => "color-dodge",
			Value::ColResize => "col-resize",
			Value::Column => "column",
			Value::ColumnReverse => "column-reverse",
			Value::Contain => "contain",
			Value::Content => "content",
			Value::ContentBox => "content-box",
			Value::Contents => "contents",
			Value::ContextMenu => "context-menu",
			Value::Copy => "copy",
			Value::Cover => "cover",
			Value::CrispEdges => "crisp-edges",
			Value::Crosshair => "crosshair",
			Value::Currentcolor => "currentcolor",
			Value::Cyclic => "cyclic",
			Value::Darken => "darken",
			Value::Dashed => "dashed",
			Value::Decimal => "decimal",
			Value::DecimalLeadingZero => "decimal-leading-zero",
			Value::Default => "default",
			Value::Deg => "deg",
			Value::Dense => "dense",
			Value::Devanagari => "devanagari",
			Value::Difference => "difference",
			Value::Disc => "disc",
			Value::DisclosureClosed => "disclosure-closed",
			Value::DisclosureOpen => "disclosure-open",
			Value::Discrete => "discrete",
			Value::Distribute => "distribute",
			Value::Dot => "dot",
			Value::Dotted => "dotted",
			Value::Double => "double",
			Value::DoubleCircle => "double-circle",
			Value::Dpcm => "dpcm",
			Value::Dpi => "dpi",
			Value::Dppx => "dppx",
			Value::Duplicate => "duplicate",
			Value::EachLine => "each-line",
			Value::Ease => "ease",
			Value::EaseIn => "ease-in",
			Value::EaseInOut => "ease-in-out",
			Value::EaseOut => "ease-out",
			Value::Ellipse => "ellipse",
			Value::Ellipsis => "ellipsis",
			Value::Em => "em",
			Value::Embed => "embed",
			Value::Embossed => "embossed",
			Value::End => "end",
			Value::EResize => "e-resize",
			Value::EthiopicNumeric => "ethiopic-numeric",
			Value::Evenodd => "evenodd",
			Value::EwResize => "ew-resize",
			Value::Ex => "ex",
			Value::Exclude => "exclude",
			Value::Exclusion => "exclusion",
			Value::Extends => "extends",
			Value::FarthestCorner => "farthest-corner",
			Value::FarthestSide => "farthest-side",
			Value::Fast => "fast",
			Value::Fill => "fill",
			Value::FillBox => "fill-box",
			Value::Filled => "filled",
			Value::Fillpaint => "fillpaint",
			Value::Fine => "fine",
			Value::First => "first",
			Value::FitContent => "fit-content",
			Value::Fixed => "fixed",
			Value::Flex => "flex",
			Value::FlexEnd => "flex-end",
			Value::FlexStart => "flex-start",
			Value::FontFeatureSettings => "font-feature-settings",
			Value::FontVariant => "font-variant",
			Value::ForceEnd => "force-end",
			Value::Forwards => "forwards",
			Value::Fr => "fr",
			Value::FromImage => "from-image",
			Value::FullSizeKana => "full-size-kana",
			Value::FullWidth => "full-width",
			Value::Gamma => "gamma",
			Value::Georgian => "georgian",
			Value::Grab => "grab",
			Value::Grabbing => "grabbing",
			Value::Grad => "grad",
			Value::Grid => "grid",
			Value::Groove => "groove",
			Value::Gujarati => "gujarati",
			Value::Gurmukhi => "gurmukhi",
			Value::Handheld => "handheld",
			Value::Hanging => "hanging",
			Value::HardLight => "hard-light",
			Value::Hebrew => "hebrew",
			Value::Help => "help",
			Value::Hidden => "hidden",
			Value::HighQuality => "high-quality",
			Value::Hiragana => "hiragana",
			Value::HiraganaIroha => "hiragana-iroha",
			Value::HorizontalTb => "horizontal-tb",
			Value::Hover => "hover",
			Value::Hue => "hue",
			Value::Hz => "hz",
			Value::Identity => "identity",
			Value::InterCharacter => "inter-character",
			Value::Interlace => "interlace",
			Value::Intersect => "intersect",
			Value::InterWord => "inter-word",
			Value::Invert => "invert",
			Value::Isolate => "isolate",
			Value::IsolateOverride => "isolate-override",
			Value::JapaneseFormal => "japanese-formal",
			Value::JapaneseInformal => "japanese-informal",
			Value::JumpBoth => "jump-both",
			Value::JumpEnd => "jump-end",
			Value::JumpNone => "jump-none",
			Value::JumpStart => "jump-start",
			Value::Justify => "justify",
			Value::JustifyAll => "justify-all",
			Value::Kannada => "kannada",
			Value::Katakana => "katakana",
			Value::KatakanaIroha => "katakana-iroha",
			Value::KeepAll => "keep-all",
			Value::Khmer => "khmer",
			Value::Khz => "khz",
			Value::KoreanHangulFormal => "korean-hangul-formal",
			Value::KoreanHanjaFormal => "korean-hanja-formal",
			Value::KoreanHanjaInformal => "korean-hanja-informal",
			Value::Landscape => "landscape",
			Value::Lao => "lao",
			Value::Last => "last",
			Value::Layout => "layout",
			Value::Left => "left",
			Value::Legacy => "legacy",
			Value::Lighten => "lighten",
			Value::Linear => "linear",
			Value::Linearrgb => "linearrgb",
			Value::LineThrough => "line-through",
			Value::ListItem => "list-item",
			Value::Local => "local",
			Value::Loose => "loose",
			Value::LowerAlpha => "lower-alpha",
			Value::LowerArmenian => "lower-armenian",
			Value::Lowercase => "lowercase",
			Value::LowerGreek => "lower-greek",
			Value::LowerLatin => "lower-latin",
			Value::LowerRoman => "lower-roman",
			Value::Ltr => "ltr",
			Value::Luminance => "luminance",
			Value::Luminosity => "luminosity",
			Value::Malayalam => "malayalam",
			Value::Mandatory => "mandatory",
			Value::Manual => "manual",
			Value::MarginBox => "margin-box",
			Value::MatchParent => "match-parent",
			Value::MatchSource => "match-source",
			Value::MaxContent => "max-content",
			Value::Medium => "medium",
			Value::MinContent => "min-content",
			Value::Minmax => "minmax",
			Value::Mixed => "mixed",
			Value::Mm => "mm",
			Value::Mongolian => "mongolian",
			Value::Move => "move",
			Value::Ms => "ms",
			Value::Multiply => "multiply",
			Value::Myanmar => "myanmar",
			Value::NeResize => "ne-resize",
			Value::NeswResize => "nesw-resize",
			Value::NoClip => "no-clip",
			Value::NoCloseQuote => "no-close-quote",
			Value::NoComposite => "no-composite",
			Value::NoDrop => "no-drop",
			Value::None => "none",
			Value::Nonzero => "nonzero",
			Value::NoOpenQuote => "no-open-quote",
			Value::NoRepeat => "no-repeat",
			Value::Normal => "normal",
			Value::Not => "not",
			Value::NotAllowed => "not-allowed",
			Value::Nowrap => "nowrap",
			Value::NResize => "n-resize",
			Value::NsResize => "ns-resize",
			Value::Numbers => "numbers",
			Value::Numeric => "numeric",
			Value::NwResize => "nw-resize",
			Value::NwseResize => "nwse-resize",
			Value::Objectboundingbox => "objectboundingbox",
			Value::Only => "only",
			Value::Open => "open",
			Value::OpenQuote => "open-quote",
			Value::OptionalPaged => "optional-paged",
			Value::Oriya => "oriya",
			Value::Outset => "outset",
			Value::Over => "over",
			Value::Overlay => "overlay",
			Value::Overline => "overline",
			Value::P3 => "p3",
			Value::PaddingBox => "padding-box",
			Value::Page => "page",
			Value::Paged => "paged",
			Value::Paint => "paint",
			Value::Paused => "paused",
			Value::Pc => "pc",
			Value::Persian => "persian",
			Value::Pixelated => "pixelated",
			Value::Pixel => "pixel",
			Value::Plaintext => "plaintext",
			Value::Pointer => "pointer",
			Value::Portrait => "portrait",
			Value::Pre => "pre",
			Value::PreLine => "pre-line",
			Value::PreWrap => "pre-wrap",
			Value::Print => "print",
			Value::Progress => "progress",
			Value::Progressive => "progressive",
			Value::Projection => "projection",
			Value::Proximity => "proximity",
			Value::Pt => "pt",
			Value::Px => "px",
			Value::Q => "q",
			Value::Rad => "rad",
			Value::Rec2020 => "rec2020",
			Value::Recto => "recto",
			Value::Region => "region",
			Value::Rem => "rem",
			Value::Repeat => "repeat",
			Value::RepeatX => "repeat-x",
			Value::RepeatY => "repeat-y",
			Value::Reverse => "reverse",
			Value::Revert => "revert",
			Value::Ridge => "ridge",
			Value::Right => "right",
			Value::Rotate => "rotate",
			Value::Round => "round",
			Value::Row => "row",
			Value::RowResize => "row-resize",
			Value::RowReverse => "row-reverse",
			Value::Rtl => "rtl",
			Value::Running => "running",
			Value::S => "s",
			Value::Safe => "safe",
			Value::Saturation => "saturation",
			Value::Scale => "scale",
			Value::ScaleDown => "scale-down",
			Value::Scalex => "scalex",
			Value::Scaley => "scaley",
			Value::Screen => "screen",
			Value::Scroll => "scroll",
			Value::ScrollPosition => "scroll-position",
			Value::SelfEnd => "self-end",
			Value::SelfStart => "self-start",
			Value::SeResize => "se-resize",
			Value::Sesame => "sesame",
			Value::Sideways => "sideways",
			Value::SidewaysRight => "sideways-right",
			Value::SimpChineseFormal => "simp-chinese-formal",
			Value::SimpChineseInformal => "simp-chinese-informal",
			Value::Size => "size",
			Value::Skew => "skew",
			Value::Skewx => "skewx",
			Value::Skewy => "skewy",
			Value::Slice => "slice",
			Value::Slow => "slow",
			Value::Smooth => "smooth",
			Value::SoftLight => "soft-light",
			Value::Solid => "solid",
			Value::Sourcealpha => "sourcealpha",
			Value::Sourcegraphic => "sourcegraphic",
			Value::Space => "space",
			Value::SpaceAround => "space-around",
			Value::SpaceBetween => "space-between",
			Value::SpaceEvenly => "space-evenly",
			Value::Speech => "speech",
			Value::SpellOut => "spell-out",
			Value::Square => "square",
			Value::SResize => "s-resize",
			Value::Srgb => "srgb",
			Value::Start => "start",
			Value::StepEnd => "step-end",
			Value::StepStart => "step-start",
			Value::Stretch => "stretch",
			Value::Strict => "strict",
			Value::StrokeBox => "stroke-box",
			Value::Strokepaint => "strokepaint",
			Value::Style => "style",
			Value::Subtract => "subtract",
			Value::SwResize => "sw-resize",
			Value::Symbolic => "symbolic",
			Value::Table => "table",
			Value::TableCaption => "table-caption",
			Value::TableCell => "table-cell",
			Value::TableColumn => "table-column",
			Value::TableColumnGroup => "table-column-group",
			Value::TableFooterGroup => "table-footer-group",
			Value::TableHeaderGroup => "table-header-group",
			Value::TableRow => "table-row",
			Value::TableRowGroup => "table-row-group",
			Value::Tamil => "tamil",
			Value::Telugu => "telugu",
			Value::Text => "text",
			Value::Thai => "thai",
			Value::Thick => "thick",
			Value::Thin => "thin",
			Value::Tibetan => "tibetan",
			Value::Top => "top",
			Value::TradChineseFormal => "trad-chinese-formal",
			Value::TradChineseInformal => "trad-chinese-informal",
			Value::Translate => "translate",
			Value::Translatex => "translatex",
			Value::Translatey => "translatey",
			Value::Triangle => "triangle",
			Value::Tty => "tty",
			Value::Turn => "turn",
			Value::Tv => "tv",
			Value::Under => "under",
			Value::Underline => "underline",
			Value::Unsafe => "unsafe",
			Value::Unset => "unset",
			Value::UpperAlpha => "upper-alpha",
			Value::UpperArmenian => "upper-armenian",
			Value::Uppercase => "uppercase",
			Value::UpperLatin => "upper-latin",
			Value::UpperRoman => "upper-roman",
			Value::Upright => "upright",
			Value::Userspaceonuse => "userspaceonuse",
			Value::Verso => "verso",
			Value::VerticalLr => "vertical-lr",
			Value::VerticalRl => "vertical-rl",
			Value::VerticalText => "vertical-text",
			Value::Vh => "vh",
			Value::ViewBox => "view-box",
			Value::Vmax => "vmax",
			Value::Vmin => "vmin",
			Value::Vw => "vw",
			Value::Wait => "wait",
			Value::Words => "words",
			Value::Wrap => "wrap",
			Value::WrapReverse => "wrap-reverse",
			Value::WResize => "w-resize",
			Value::X => "x",
			Value::Xor => "xor",
			Value::Y => "y",
			Value::ZoomIn => "zoom-in",
			Value::ZoomOut => "zoom-out",
		};
		format!("{prop}: {value}", prop=property, value=value)
	}
}
