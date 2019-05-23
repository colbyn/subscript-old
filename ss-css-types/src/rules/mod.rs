// REF: https://www.w3.org/TR/CSS/#indices
pub mod syntax;
pub use crate::values::*;


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Rule {
	pub property: Property,
	pub value: Value,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Property {
	AlignContent,
	AlignItems,
	AlignSelf,
	All,
	Animation,
	AnimationDelay,
	AnimationDirection,
	AnimationDuration,
	AnimationFillMode,
	AnimationIterationCount,
	AnimationName,
	AnimationPlayState,
	AnimationTimingFunction,
	Azimuth,
	Background,
	BackgroundAttachment,
	BackgroundBlendMode,
	BackgroundClip,
	BackgroundColor,
	BackgroundImage,
	BackgroundOrigin,
	BackgroundPosition,
	BackgroundRepeat,
	BackgroundSize,
	Border,
	BorderBottom,
	BorderBottomColor,
	BorderBottomLeftRadius,
	BorderBottomRightRadius,
	BorderBottomStyle,
	BorderBottomWidth,
	BorderCollapse,
	BorderColor,
	BorderImage,
	BorderImageOutset,
	BorderImageRepeat,
	BorderImageSlice,
	BorderImageSource,
	BorderImageWidth,
	BorderLeft,
	BorderLeftColor,
	BorderLeftStyle,
	BorderLeftWidth,
	BorderRadius,
	BorderRight,
	BorderRightColor,
	BorderRightStyle,
	BorderRightWidth,
	BorderSpacing,
	BorderStyle,
	BorderTop,
	BorderTopColor,
	BorderTopLeftRadius,
	BorderTopRightRadius,
	BorderTopStyle,
	BorderTopWidth,
	BorderWidth,
	Bottom,
	BoxDecorationBreak,
	BoxShadow,
	BoxSizing,
	BreakAfter,
	BreakBefore,
	BreakInside,
	CaptionSide,
	CaretColor,
	Clear,
	Clip,
	ClipPath,
	ClipRule,
	Color,
	ColorInterpolationFilters,
	ColumnCount,
	ColumnFill,
	ColumnGap,
	ColumnRule,
	ColumnRuleColor,
	ColumnRuleStyle,
	ColumnRuleWidth,
	Columns,
	ColumnSpan,
	ColumnWidth,
	Contain,
	Content,
	CounterIncrement,
	CounterReset,
	Cue,
	CueAfter,
	CueBefore,
	Cursor,
	Direction,
	Display,
	Elevation,
	EmptyCells,
	Filter,
	Flex,
	FlexBasis,
	FlexDirection,
	FlexFlow,
	FlexGrow,
	FlexShrink,
	FlexWrap,
	Float,
	FloodColor,
	FloodOpacity,
	Font,
	FontFamily,
	FontFeatureSettings,
	FontKerning,
	FontSize,
	FontSizeAdjust,
	FontStretch,
	FontStyle,
	FontSynthesis,
	FontVariant,
	FontVariantCaps,
	FontVariantEastAsian,
	FontVariantLigatures,
	FontVariantNumeric,
	FontVariantPosition,
	FontWeight,
	Gap,
	Globalcompositeoperation,
	GlyphOrientationVertical,
	Grid,
	GridArea,
	GridAutoColumns,
	GridAutoFlow,
	GridAutoRows,
	GridColumn,
	GridColumnEnd,
	GridColumnGap,
	GridColumnStart,
	GridGap,
	GridRow,
	GridRowEnd,
	GridRowGap,
	GridRowStart,
	GridTemplate,
	GridTemplateAreas,
	GridTemplateColumns,
	GridTemplateRows,
	HangingPunctuation,
	Height,
	Hyphens,
	ImageOrientation,
	ImageRendering,
	ImageResolution,
	Isolation,
	JustifyContent,
	JustifyItems,
	JustifySelf,
	Left,
	LetterSpacing,
	LightingColor,
	LineBreak,
	LineHeight,
	ListStyle,
	ListStyleImage,
	ListStylePosition,
	ListStyleType,
	Margin,
	MarginBottom,
	MarginLeft,
	MarginRight,
	MarginTop,
	Mask,
	MaskBorder,
	MaskBorderMode,
	MaskBorderOutset,
	MaskBorderRepeat,
	MaskBorderSlice,
	MaskBorderSource,
	MaskBorderWidth,
	MaskClip,
	MaskComposite,
	MaskImage,
	MaskMode,
	MaskOrigin,
	MaskPosition,
	MaskRepeat,
	MaskSize,
	MaskType,
	MaxHeight,
	MaxWidth,
	MinHeight,
	MinWidth,
	MixBlendMode,
	ObjectFit,
	ObjectPosition,
	Opacity,
	Order,
	Orphans,
	Outline,
	OutlineColor,
	OutlineOffset,
	OutlineStyle,
	OutlineWidth,
	Overflow,
	OverflowWrap,
	Padding,
	PaddingBottom,
	PaddingLeft,
	PaddingRight,
	PaddingTop,
	PageBreakAfter,
	PageBreakBefore,
	PageBreakInside,
	Pause,
	PauseAfter,
	PauseBefore,
	Pitch,
	PitchRange,
	PlaceContent,
	PlaceItems,
	PlaceSelf,
	PlayDuring,
	Position,
	Quotes,
	Resize,
	Rest,
	RestAfter,
	RestBefore,
	Richness,
	Right,
	RowGap,
	ScrollMargin,
	ScrollMarginBlock,
	ScrollMarginBlockEnd,
	ScrollMarginBlockStart,
	ScrollMarginBottom,
	ScrollMarginInline,
	ScrollMarginInlineEnd,
	ScrollMarginInlineStart,
	ScrollMarginLeft,
	ScrollMarginRight,
	ScrollMarginTop,
	ScrollPadding,
	ScrollPaddingBlock,
	ScrollPaddingBlockEnd,
	ScrollPaddingBlockStart,
	ScrollPaddingBottom,
	ScrollPaddingInline,
	ScrollPaddingInlineEnd,
	ScrollPaddingInlineStart,
	ScrollPaddingLeft,
	ScrollPaddingRight,
	ScrollPaddingTop,
	ScrollSnapAlign,
	ScrollSnapStop,
	ScrollSnapType,
	ShapeImageThreshold,
	ShapeMargin,
	ShapeOutside,
	Speak,
	SpeakAs,
	SpeakHeader,
	SpeakNumeral,
	SpeakPunctuation,
	SpeechRate,
	Stress,
	TableLayout,
	TabSize,
	TextAlign,
	TextAlignAll,
	TextAlignLast,
	TextCombineUpright,
	TextDecoration,
	TextDecorationColor,
	TextDecorationLine,
	TextDecorationStyle,
	TextEmphasis,
	TextEmphasisColor,
	TextEmphasisPosition,
	TextEmphasisStyle,
	TextIndent,
	TextJustify,
	TextOrientation,
	TextOverflow,
	TextShadow,
	TextTransform,
	TextUnderlinePosition,
	Top,
	Transform,
	TransformBox,
	TransformOrigin,
	Transition,
	TransitionDelay,
	TransitionDuration,
	TransitionProperty,
	TransitionTimingFunction,
	UnicodeBidi,
	VerticalAlign,
	Visibility,
	VoiceBalance,
	VoiceDuration,
	VoiceFamily,
	VoicePitch,
	VoiceRange,
	VoiceRate,
	VoiceStress,
	VoiceVolume,
	Volume,
	WhiteSpace,
	Widows,
	Width,
	WillChange,
	WordBreak,
	WordSpacing,
	WordWrap,
	WritingMode,
	ZIndex,
}






