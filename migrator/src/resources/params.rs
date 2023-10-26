/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

struct ComparisonParam<'a> {
    resources: &'a ComparisonsInit<'a>,
}

impl<'a> DbResourcesMeta<Params> for ComparisonParam<'a> {
    fn get_left(&self) -> Params {
        self.resources.left_resources.params()
    }

    fn get_right(&self) -> Params {
        self.resources.right_resources.params()
    }
}

struct ComparisonParam<'a> {
    resources: &'a ComparisonsInit<'a>,
}

impl<'a> DbResourcesMeta<Params> for ComparisonParam<'a> {
    fn get_left(&self) -> Params {
        self.resources.left_resources.params()
    }

    fn get_right(&self) -> Params {
        self.resources.right_resources.params()
    }
}
