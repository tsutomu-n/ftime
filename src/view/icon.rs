use crate::model::TimeBucket;

/// アイコン取得のためのインタフェース。
pub trait IconProvider {
    fn bucket_icon(&self, bucket: TimeBucket) -> &'static str;
}

/// デフォルトではアイコンを出さない。
pub struct DefaultIconProvider;

impl IconProvider for DefaultIconProvider {
    fn bucket_icon(&self, _bucket: TimeBucket) -> &'static str {
        ""
    }
}

/// Nerd Fontsを想定したアイコンセット。
/// feature `icons` を有効にしたビルド時だけ使用され、未導入環境でも機能が壊れないように
/// main側でフォールバックする。
#[cfg(feature = "icons")]
pub struct NerdIconProvider;

#[cfg(feature = "icons")]
impl IconProvider for NerdIconProvider {
    fn bucket_icon(&self, bucket: TimeBucket) -> &'static str {
        // 代表的なNerd Fontグリフ。フォント未導入でも文字列は空にならず、
        // レイアウト崩れを避けるため単一文字を返す。
        match bucket {
            TimeBucket::Active => "",   // flame-like
            TimeBucket::Today => "󰣿",    // coffee cup
            TimeBucket::ThisWeek => "󱞁", // calendar
            TimeBucket::History => "󱎓",  // history/clock
        }
    }
}
