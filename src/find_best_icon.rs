use super::Icon;

fn sort_icons_by_size(icons: &mut Vec<Icon>) {
    icons.sort_by(|a, b| b.size.cmp(&a.size));
}

pub fn find_best_icon(icons: &mut Vec<Icon>, ext: Option<&str>) -> Option<Icon> {
    sort_icons_by_size(icons);

    if let Some(ext) = ext {
        for icon in icons.iter() {
            if icon.ext == ext {
                return Some(Icon{
                    source: icon.source.clone(),
                    data: icon.data.clone(),
                    size: icon.size,
                    ext: icon.ext.clone(),
                    mime: icon.mime.clone(),
                });
            }
        }
    }

    if icons.len() > 0 {
        Some(Icon{
            source: icons[0].source.clone(),
            data: icons[0].data.clone(),
            size: icons[0].size,
            ext: icons[0].ext.clone(),
            mime: icons[0].mime.clone(),
        })
    } else {
        None
    }
}
