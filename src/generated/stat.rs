use crate::*;
#[allow(unused_imports)]
use crate::stat::*;

#[inline]
pub fn avedev<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("AVEDEV", n)
}

#[inline]
pub fn average<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("AVERAGE", n)
}

#[inline]
pub fn averagea<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("AVERAGEA", n)
}

#[inline]
pub fn averageif<A: Reference, B: Criterion>(r: A, c: B) -> FnNumber2<A, B> {
    FnNumber2("AVERAGEIF", r, c)
}

#[inline]
pub fn averageif_<A: Reference, B: Criterion, C: Reference>(r: A, c: B, a: C) -> FnNumber3<A, B, C> {
    FnNumber3("AVERAGEIF", r, c, a)
}

#[inline]
pub fn betadist<A: Number, B: Number, C: Number>(x: A, alpha: B, beta: C) -> FnNumber3<A, B, C> {
    FnNumber3("BETADIST", x, alpha, beta)
}

#[inline]
pub fn betadist_<A: Number, B: Number, C: Number, D: Number>(x: A, alpha: B, beta: C, a: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BETADIST", x, alpha, beta, a)
}

#[inline]
pub fn betadist__<A: Number, B: Number, C: Number, D: Number, E: Number>(x: A, alpha: B, beta: C, a: D, b: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("BETADIST", x, alpha, beta, a, b)
}

#[inline]
pub fn betadist___<A: Number, B: Number, C: Number, D: Number, E: Number, F: Logical>(x: A, alpha: B, beta: C, a: D, b: E, cumulative: F) -> FnNumber6<A, B, C, D, E, F> {
    FnNumber6("BETADIST", x, alpha, beta, a, b, cumulative)
}

#[inline]
pub fn betainv<A: Number, B: Number, C: Number>(p: A, alpha: B, beta: C) -> FnNumber3<A, B, C> {
    FnNumber3("BETAINV", p, alpha, beta)
}

#[inline]
pub fn betainv_<A: Number, B: Number, C: Number, D: Number>(p: A, alpha: B, beta: C, a: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BETAINV", p, alpha, beta, a)
}

#[inline]
pub fn betainv__<A: Number, B: Number, C: Number, D: Number, E: Number>(p: A, alpha: B, beta: C, a: D, b: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("BETAINV", p, alpha, beta, a, b)
}

#[inline]
pub fn binom_dist_range<A: Number, B: Number, C: Number>(n: A, p: B, s: C) -> FnNumber3<A, B, C> {
    FnNumber3("BINOM.DIST.RANGE", n, p, s)
}

#[inline]
pub fn binom_dist_range_<A: Number, B: Number, C: Number, D: Number>(n: A, p: B, s: C, s2: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BINOM.DIST.RANGE", n, p, s, s2)
}

#[inline]
pub fn binomdist<A: Number, B: Number, C: Number, D: Logical>(s: A, n: B, p: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BINOMDIST", s, n, p, cumulative)
}

#[inline]
pub fn legacy_chidist<A: Number, B: Number>(x: A, degrees_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("LEGACY.CHIDIST", x, degrees_of_freedom)
}

#[inline]
pub fn chisqdist<A: Number, B: Number>(x: A, degrees_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("CHISQDIST", x, degrees_of_freedom)
}

#[inline]
pub fn chisqdist_<A: Number, B: Number, C: Logical>(x: A, degrees_of_freedom: B, cumulative: C) -> FnNumber3<A, B, C> {
    FnNumber3("CHISQDIST", x, degrees_of_freedom, cumulative)
}

#[inline]
pub fn legacy_chiinv<A: Number, B: Number>(p: A, degrees_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("LEGACY.CHIINV", p, degrees_of_freedom)
}

#[inline]
pub fn chisqinv<A: Number, B: Number>(p: A, degrees_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("CHISQINV", p, degrees_of_freedom)
}

#[inline]
pub fn legacy_chitest<A: Array, B: Array>(a: A, e: B) -> FnNumber2<A, B> {
    FnNumber2("LEGACY.CHITEST", a, e)
}

#[inline]
pub fn confidence<A: Number, B: Number, C: Number>(alpha: A, stddev: B, size: C) -> FnNumber3<A, B, C> {
    FnNumber3("CONFIDENCE", alpha, stddev, size)
}

#[inline]
pub fn correl<A: Array, B: Array>(n1: A, n2: B) -> FnNumber2<A, B> {
    FnNumber2("CORREL", n1, n2)
}

#[inline]
pub fn covar<A: Array, B: Array>(n1: A, n2: B) -> FnNumber2<A, B> {
    FnNumber2("COVAR", n1, n2)
}

#[inline]
pub fn critbinom<A: Number, B: Number, C: Number>(trials: A, s_p: B, alpha: C) -> FnNumber3<A, B, C> {
    FnNumber3("CRITBINOM", trials, s_p, alpha)
}

#[inline]
pub fn devsq<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("DEVSQ", n)
}

#[inline]
pub fn expondist<A: Number, B: Number>(x: A, λ: B) -> FnNumber2<A, B> {
    FnNumber2("EXPONDIST", x, λ)
}

#[inline]
pub fn expondist_<A: Number, B: Number, C: Logical>(x: A, λ: B, cumulative: C) -> FnNumber3<A, B, C> {
    FnNumber3("EXPONDIST", x, λ, cumulative)
}

#[inline]
pub fn fdist<A: Number, B: Number, C: Number>(x: A, r1: B, r2: C) -> FnNumber3<A, B, C> {
    FnNumber3("FDIST", x, r1, r2)
}

#[inline]
pub fn fdist_<A: Number, B: Number, C: Number, D: Logical>(x: A, r1: B, r2: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("FDIST", x, r1, r2, cumulative)
}

#[inline]
pub fn legacy_fdist<A: Number, B: Number, C: Number>(x: A, r1: B, r2: C) -> FnNumber3<A, B, C> {
    FnNumber3("LEGACY.FDIST", x, r1, r2)
}

#[inline]
pub fn finv<A: Number, B: Number, C: Number>(p: A, r1: B, r2: C) -> FnNumber3<A, B, C> {
    FnNumber3("FINV", p, r1, r2)
}

#[inline]
pub fn legacy_finv<A: Number, B: Number, C: Number>(p: A, r1: B, r2: C) -> FnNumber3<A, B, C> {
    FnNumber3("LEGACY.FINV", p, r1, r2)
}

#[inline]
pub fn fisher<A: Number>(r: A) -> FnNumber1<A> {
    FnNumber1("FISHER", r)
}

#[inline]
pub fn fisherinv<A: Number>(r: A) -> FnNumber1<A> {
    FnNumber1("FISHERINV", r)
}

#[inline]
pub fn forecast<A: Number, B: Array, C: Array>(value: A, data_y: B, data_x: C) -> FnNumber3<A, B, C> {
    FnNumber3("FORECAST", value, data_y, data_x)
}

#[inline]
pub fn frequency<A: Sequence, B: Sequence>(data: A, bins: B) -> FnArray2<A, B> {
    FnArray2("FREQUENCY", data, bins)
}

#[inline]
pub fn ftest<A: Sequence, B: Sequence>(data_1: A, data_2: B) -> FnNumber2<A, B> {
    FnNumber2("FTEST", data_1, data_2)
}

#[inline]
pub fn gammadist<A: Number, B: Number, C: Number>(x: A, alpha: B, beta: C) -> FnNumber3<A, B, C> {
    FnNumber3("GAMMADIST", x, alpha, beta)
}

#[inline]
pub fn gammadist_<A: Number, B: Number, C: Number, D: Logical>(x: A, alpha: B, beta: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("GAMMADIST", x, alpha, beta, cumulative)
}

#[inline]
pub fn gammainv<A: Number, B: Number, C: Number>(p: A, alpha: B, beta: C) -> FnNumber3<A, B, C> {
    FnNumber3("GAMMAINV", p, alpha, beta)
}

#[inline]
pub fn gauss<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("GAUSS", x)
}

#[inline]
pub fn geomean<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("GEOMEAN", n)
}

#[inline]
pub fn growth<A: Array>(known_y: A) -> FnArray1<A> {
    FnArray1("GROWTH", known_y)
}

#[inline]
pub fn growth_<A: Array, B: Array>(known_y: A, known_x: B) -> FnArray2<A, B> {
    FnArray2("GROWTH", known_y, known_x)
}

#[inline]
pub fn growth__<A: Array, B: Array, C: Array>(known_y: A, known_x: B, new_x: C) -> FnArray3<A, B, C> {
    FnArray3("GROWTH", known_y, known_x, new_x)
}

#[inline]
pub fn growth___<A: Array, B: Array, C: Array, D: Logical>(known_y: A, known_x: B, new_x: C, const_: D) -> FnArray4<A, B, C, D> {
    FnArray4("GROWTH", known_y, known_x, new_x, const_)
}

#[inline]
pub fn harmean<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("HARMEAN", n)
}

#[inline]
pub fn hypgeomdist<A: Number, B: Number, C: Number, D: Number>(x: A, t: B, m: C, n: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("HYPGEOMDIST", x, t, m, n)
}

#[inline]
pub fn hypgeomdist_<A: Number, B: Number, C: Number, D: Number, E: Logical>(x: A, t: B, m: C, n: D, cumulative: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("HYPGEOMDIST", x, t, m, n, cumulative)
}

#[inline]
pub fn intercept<A: Array, B: Array>(data_y: A, data_x: B) -> FnNumber2<A, B> {
    FnNumber2("INTERCEPT", data_y, data_x)
}

#[inline]
pub fn kurt<A: Sequence>(x: A) -> FnNumber1<A> {
    FnNumber1("KURT", x)
}

#[inline]
pub fn large<A: Sequence, B: NumberOrArray>(list: A, n: B) -> FnArray2<A, B> {
    FnArray2("LARGE", list, n)
}

#[inline]
pub fn linest<A: Array>(known_y: A) -> FnArray1<A> {
    FnArray1("LINEST", known_y)
}

#[inline]
pub fn linest_<A: Array, B: Array>(known_y: A, known_x: B) -> FnArray2<A, B> {
    FnArray2("LINEST", known_y, known_x)
}

#[inline]
pub fn linest__<A: Array, B: Array, C: Logical>(known_y: A, known_x: B, const_: C) -> FnArray3<A, B, C> {
    FnArray3("LINEST", known_y, known_x, const_)
}

#[inline]
pub fn linest___<A: Array, B: Array, C: Logical, D: Logical>(known_y: A, known_x: B, const_: C, stats: D) -> FnArray4<A, B, C, D> {
    FnArray4("LINEST", known_y, known_x, const_, stats)
}

#[inline]
pub fn logest<A: Array>(known_y: A) -> FnArray1<A> {
    FnArray1("LOGEST", known_y)
}

#[inline]
pub fn logest_<A: Array, B: Array>(known_y: A, known_x: B) -> FnArray2<A, B> {
    FnArray2("LOGEST", known_y, known_x)
}

#[inline]
pub fn logest__<A: Array, B: Array, C: Logical>(known_y: A, known_x: B, const_: C) -> FnArray3<A, B, C> {
    FnArray3("LOGEST", known_y, known_x, const_)
}

#[inline]
pub fn logest___<A: Array, B: Array, C: Logical, D: Logical>(known_y: A, known_x: B, const_: C, stats: D) -> FnArray4<A, B, C, D> {
    FnArray4("LOGEST", known_y, known_x, const_, stats)
}

#[inline]
pub fn loginv<A: Number>(p: A) -> FnNumber1<A> {
    FnNumber1("LOGINV", p)
}

#[inline]
pub fn loginv_<A: Number, B: Number>(p: A, mean: B) -> FnNumber2<A, B> {
    FnNumber2("LOGINV", p, mean)
}

#[inline]
pub fn loginv__<A: Number, B: Number, C: Number>(p: A, mean: B, standard_deviation: C) -> FnNumber3<A, B, C> {
    FnNumber3("LOGINV", p, mean, standard_deviation)
}

#[inline]
pub fn lognormdist<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("LOGNORMDIST", x)
}

#[inline]
pub fn lognormdist_<A: Number, B: Number>(x: A, μ: B) -> FnNumber2<A, B> {
    FnNumber2("LOGNORMDIST", x, μ)
}

#[inline]
pub fn lognormdist__<A: Number, B: Number, C: Number>(x: A, μ: B, σ: C) -> FnNumber3<A, B, C> {
    FnNumber3("LOGNORMDIST", x, μ, σ)
}

#[inline]
pub fn lognormdist___<A: Number, B: Number, C: Number, D: Logical>(x: A, μ: B, σ: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("LOGNORMDIST", x, μ, σ, cumulative)
}

#[inline]
pub fn max<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MAX", n)
}

#[inline]
pub fn maxa<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MAXA", n)
}

#[inline]
pub fn median<A: Sequence>(x: A) -> FnNumber1<A> {
    FnNumber1("MEDIAN", x)
}

#[inline]
pub fn min<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MIN", n)
}

#[inline]
pub fn mina<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MINA", n)
}

#[inline]
pub fn mode<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MODE", n)
}

#[inline]
pub fn negbinomdist<A: Number, B: Number, C: Number>(x: A, r: B, prob: C) -> FnNumber3<A, B, C> {
    FnNumber3("NEGBINOMDIST", x, r, prob)
}

#[inline]
pub fn normdist<A: Number, B: Number, C: Number>(x: A, mean: B, standard_deviation: C) -> FnNumber3<A, B, C> {
    FnNumber3("NORMDIST", x, mean, standard_deviation)
}

#[inline]
pub fn normdist_<A: Number, B: Number, C: Number, D: Logical>(x: A, mean: B, standard_deviation: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("NORMDIST", x, mean, standard_deviation, cumulative)
}

#[inline]
pub fn norminv<A: Number, B: Number, C: Number>(p: A, mean: B, standard_deviation: C) -> FnNumber3<A, B, C> {
    FnNumber3("NORMINV", p, mean, standard_deviation)
}

#[inline]
pub fn legacy_normsdist<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("LEGACY.NORMSDIST", x)
}

#[inline]
pub fn legacy_normsinv<A: Number>(p: A) -> FnNumber1<A> {
    FnNumber1("LEGACY.NORMSINV", p)
}

#[inline]
pub fn pearson<A: Array, B: Array>(independent_values: A, dependent_values: B) -> FnNumber2<A, B> {
    FnNumber2("PEARSON", independent_values, dependent_values)
}

#[inline]
pub fn percentile<A: Sequence, B: Number>(data: A, x: B) -> FnNumber2<A, B> {
    FnNumber2("PERCENTILE", data, x)
}

#[inline]
pub fn percentrank<A: Sequence, B: Number>(data: A, x: B) -> FnNumber2<A, B> {
    FnNumber2("PERCENTRANK", data, x)
}

#[inline]
pub fn percentrank_<A: Sequence, B: Number, C: Number>(data: A, x: B, significance: C) -> FnNumber3<A, B, C> {
    FnNumber3("PERCENTRANK", data, x, significance)
}

#[inline]
pub fn permut<A: Number, B: Number>(n: A, k: B) -> FnNumber2<A, B> {
    FnNumber2("PERMUT", n, k)
}

#[inline]
pub fn permutationa<A: Number, B: Number>(total: A, chosen: B) -> FnNumber2<A, B> {
    FnNumber2("PERMUTATIONA", total, chosen)
}

#[inline]
pub fn phi<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("PHI", n)
}

#[inline]
pub fn poisson<A: Number, B: Number>(x: A, λ: B) -> FnNumber2<A, B> {
    FnNumber2("POISSON", x, λ)
}

#[inline]
pub fn poisson_<A: Number, B: Number, C: Logical>(x: A, λ: B, cumulative: C) -> FnNumber3<A, B, C> {
    FnNumber3("POISSON", x, λ, cumulative)
}

#[inline]
pub fn prob<A: Array, B: Array, C: Number>(data: A, probability: B, start: C) -> FnNumber3<A, B, C> {
    FnNumber3("PROB", data, probability, start)
}

#[inline]
pub fn prob_<A: Array, B: Array, C: Number, D: Number>(data: A, probability: B, start: C, end: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("PROB", data, probability, start, end)
}

#[inline]
pub fn quartile<A: Sequence, B: Number>(data: A, quart: B) -> FnNumber2<A, B> {
    FnNumber2("QUARTILE", data, quart)
}

#[inline]
pub fn rank<A: Number, B: Sequence>(value: A, data: B) -> FnNumber2<A, B> {
    FnNumber2("RANK", value, data)
}

#[inline]
pub fn rank_<A: Number, B: Sequence, C: Number>(value: A, data: B, order: C) -> FnNumber3<A, B, C> {
    FnNumber3("RANK", value, data, order)
}

#[inline]
pub fn rsq<A: Array, B: Array>(array_y: A, array_x: B) -> FnNumber2<A, B> {
    FnNumber2("RSQ", array_y, array_x)
}

#[inline]
pub fn skew<A: Sequence>(sample: A) -> FnNumber1<A> {
    FnNumber1("SKEW", sample)
}

#[inline]
pub fn skewp<A: Sequence>(population: A) -> FnNumber1<A> {
    FnNumber1("SKEWP", population)
}

#[inline]
pub fn slope<A: Array, B: Array>(y: A, x: B) -> FnNumber2<A, B> {
    FnNumber2("SLOPE", y, x)
}

#[inline]
pub fn small<A: Sequence, B: NumberOrArray>(list: A, n: B) -> FnArray2<A, B> {
    FnArray2("SMALL", list, n)
}

#[inline]
pub fn standardize<A: Number, B: Number, C: Number>(value: A, mean: B, sigma: C) -> FnNumber3<A, B, C> {
    FnNumber3("STANDARDIZE", value, mean, sigma)
}

#[inline]
pub fn stdev<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("STDEV", n)
}

#[inline]
pub fn stdeva<A: Sequence>(sample: A) -> FnNumber1<A> {
    FnNumber1("STDEVA", sample)
}

#[inline]
pub fn stdevp<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("STDEVP", n)
}

#[inline]
pub fn stdevpa<A: Sequence>(sample: A) -> FnNumber1<A> {
    FnNumber1("STDEVPA", sample)
}

#[inline]
pub fn steyx<A: Array, B: Array>(measured_y: A, x: B) -> FnNumber2<A, B> {
    FnNumber2("STEYX", measured_y, x)
}

#[inline]
pub fn legacy_tdist<A: Number, B: Number, C: Number>(x: A, df: B, tails: C) -> FnNumber3<A, B, C> {
    FnNumber3("LEGACY.TDIST", x, df, tails)
}

#[inline]
pub fn tinv<A: Number, B: Number>(probability: A, degree_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("TINV", probability, degree_of_freedom)
}

#[inline]
pub fn trend<A: Array>(known_y: A) -> FnArray1<A> {
    FnArray1("TREND", known_y)
}

#[inline]
pub fn trend_<A: Array, B: Array>(known_y: A, known_x: B) -> FnArray2<A, B> {
    FnArray2("TREND", known_y, known_x)
}

#[inline]
pub fn trend__<A: Array, B: Array, C: Array>(known_y: A, known_x: B, new_x: C) -> FnArray3<A, B, C> {
    FnArray3("TREND", known_y, known_x, new_x)
}

#[inline]
pub fn trend___<A: Array, B: Array, C: Array, D: Logical>(known_y: A, known_x: B, new_x: C, const_: D) -> FnArray4<A, B, C, D> {
    FnArray4("TREND", known_y, known_x, new_x, const_)
}

#[inline]
pub fn trimmean<A: Sequence, B: Number>(data_set: A, cut_off_fraction: B) -> FnNumber2<A, B> {
    FnNumber2("TRIMMEAN", data_set, cut_off_fraction)
}

#[inline]
pub fn ttest<A: Array, B: Array, C: Number, D: Number>(x: A, y: B, tails: C, type_: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("TTEST", x, y, tails, type_)
}

#[inline]
pub fn var<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("VAR", n)
}

#[inline]
pub fn vara<A: Sequence>(sample: A) -> FnNumber1<A> {
    FnNumber1("VARA", sample)
}

#[inline]
pub fn varp<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("VARP", n)
}

#[inline]
pub fn varpa<A: Sequence>(sample: A) -> FnNumber1<A> {
    FnNumber1("VARPA", sample)
}

#[inline]
pub fn weibull<A: Number, B: Number, C: Number, D: Logical>(value: A, shape: B, scale: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("WEIBULL", value, shape, scale, cumulative)
}

#[inline]
pub fn ztest<A: Sequence, B: Number>(sample: A, mean: B) -> FnNumber2<A, B> {
    FnNumber2("ZTEST", sample, mean)
}

#[inline]
pub fn ztest_<A: Sequence, B: Number, C: Number>(sample: A, mean: B, sigma: C) -> FnNumber3<A, B, C> {
    FnNumber3("ZTEST", sample, mean, sigma)
}
