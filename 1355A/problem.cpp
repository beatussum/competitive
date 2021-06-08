#include <iostream>
#include <limits>

// BEGIN: some custom utilities

namespace me
{
    template <typename _Integer0>
    class IntegerWrapper
    {
        static_assert(std::is_integral_v<_Integer0>);

    public:
        using underlying_type = _Integer0;
    public:
        constexpr IntegerWrapper() noexcept = default;

        constexpr IntegerWrapper(const _Integer0 __i) noexcept
            : m_integer_(__i)
        {}

        template <typename _Integer1>
        constexpr operator IntegerWrapper<_Integer1>() noexcept { return m_integer_; }

        constexpr operator _Integer0() const noexcept { return m_integer_; }
    public:
        constexpr IntegerWrapper operator+=(const _Integer0 __r) noexcept
            { return m_integer_ += __r; }

        constexpr IntegerWrapper operator-=(const _Integer0 __r) noexcept
            { return m_integer_ -= __r; }

        constexpr IntegerWrapper operator*=(const _Integer0 __r) noexcept
            { return m_integer_ *= __r; }

        constexpr IntegerWrapper operator/=(const _Integer0 __r) noexcept
            { return m_integer_ /= __r; }

        constexpr IntegerWrapper operator%=(const _Integer0 __r) noexcept
            { return m_integer_ %= __r; }

        constexpr IntegerWrapper operator&=(const _Integer0 __r) noexcept
            { return m_integer_ &= __r; }

        constexpr IntegerWrapper operator|=(const _Integer0 __r) noexcept
            { return m_integer_ |= __r; }

        constexpr IntegerWrapper operator^=(const _Integer0 __r) noexcept
            { return m_integer_ ^= __r; }

        constexpr IntegerWrapper operator<<=(const _Integer0 __r) noexcept
            { return m_integer_ <<= __r; }

        constexpr IntegerWrapper operator>>=(const _Integer0 __r) noexcept
            { return m_integer_ >>= __r; }

        constexpr IntegerWrapper operator++() noexcept { return ++m_integer_; }
        constexpr IntegerWrapper operator--() noexcept { return --m_integer_; }
        constexpr IntegerWrapper operator++(int) noexcept { return m_integer_++; }
        constexpr IntegerWrapper operator--(int) noexcept { return m_integer_--; }
    private:
        _Integer0 m_integer_;
    };

    using int8_t = IntegerWrapper<std::int8_t>;
    using uint8_t = IntegerWrapper<std::uint8_t>;
    using int_fast8_t = IntegerWrapper<std::int_fast8_t>;
    using uint_fast8_t = IntegerWrapper<std::uint_fast8_t>;

    namespace literals
    {
        constexpr me::int8_t operator ""_i8(unsigned long long __l) noexcept { return __l; }
        constexpr me::uint8_t operator ""_ui8(unsigned long long __l) noexcept { return __l; }
        constexpr me::int_fast8_t operator ""_if8(unsigned long long __l) noexcept { return __l; }
        constexpr me::uint_fast8_t operator ""_uif8(unsigned long long __l) noexcept { return __l; }

        constexpr std::int16_t operator ""_i16(unsigned long long __l) noexcept { return __l; }
        constexpr std::uint16_t operator ""_ui16(unsigned long long __l) noexcept { return __l; }
        constexpr std::int_fast16_t operator ""_if16(unsigned long long __l) noexcept { return __l; }
        constexpr std::uint_fast16_t operator ""_uif16(unsigned long long __l) noexcept { return __l; }

        constexpr std::int32_t operator ""_i32(unsigned long long __l) noexcept { return __l; }
        constexpr std::uint32_t operator ""_ui32(unsigned long long __l) noexcept { return __l; }
        constexpr std::int_fast32_t operator ""_if32(unsigned long long __l) noexcept { return __l; }
        constexpr std::uint_fast32_t operator ""_uif32(unsigned long long __l) noexcept { return __l; }

        constexpr std::int64_t operator ""_i64(unsigned long long __l) noexcept { return __l; }
        constexpr std::uint64_t operator ""_ui64(unsigned long long __l) noexcept { return __l; }
        constexpr std::int_fast64_t operator ""_if64(unsigned long long __l) noexcept { return __l; }
        constexpr std::uint_fast64_t operator ""_uif64(unsigned long long __l) noexcept { return __l; }
    }

    template <class _CharT, class _Traits, typename _Integer>
    std::basic_ostream<_CharT, _Traits>&
    operator<<(std::basic_ostream<_CharT, _Traits>& __o, IntegerWrapper<_Integer> __n)
    {
        switch (__o.flags() & std::ios::basefield) {
            case std::ios_base::hex:
            case std::ios_base::oct:
                return (__o << static_cast<long>(static_cast<std::make_unsigned_t<_Integer>>(__n)));
            default:
                return (__o << static_cast<long>(__n));
        }
    }

    template <class _CharT, class _Traits, typename _Integer>
    std::basic_istream<_CharT, _Traits>&
    operator>>(std::basic_istream<_CharT, _Traits>& __i, IntegerWrapper<_Integer>& __n)
    {
        using num_get = std::num_get<_CharT, std::istreambuf_iterator<_CharT, _Traits>>;
        using sentry = typename std::basic_istream<_CharT, _Traits>::sentry;

        sentry s(__i);

        if (s) {
            auto err = std::ios_base::goodbit;

            try {
                long l;
                std::use_facet<num_get>(__i.getloc()).get(__i, 0, __i, err, l);

                if (l < std::numeric_limits<_Integer>::min()) {
                    err |= std::ios_base::failbit;
                    __n = std::numeric_limits<_Integer>::min();
                } else if (l > std::numeric_limits<_Integer>::max()) {
                    err |= std::ios_base::failbit;
                    __n = std::numeric_limits<_Integer>::max();
                } else {
                    __n = static_cast<_Integer>(l);
                }
#ifdef __GLIBCXX__
            } catch (__cxxabiv1::__forced_unwind&) {
                __i.setstate(std::ios_base::badbit);
                throw;
#endif
            } catch (...) {
                __i.setstate(std::ios_base::badbit);
            }

            if (err) {
                __i.setstate(err);
            }
        }

        return __i;
    }
}

template <typename _Integer>
class std::numeric_limits<me::IntegerWrapper<_Integer>>
    : public std::numeric_limits<_Integer>
{
private:
    using _base_t = std::numeric_limits<_Integer>;
public:
    static constexpr me::IntegerWrapper<_Integer> min() noexcept
        { return _base_t::min(); }

    static constexpr me::IntegerWrapper<_Integer> max() noexcept
        { return _base_t::max(); }

    static constexpr me::IntegerWrapper<_Integer> lowest() noexcept
        { return _base_t::lowest(); }

    static constexpr me::IntegerWrapper<_Integer> epsilon() noexcept
        { return _base_t::epsilon(); }

    static constexpr me::IntegerWrapper<_Integer> round_error() noexcept
        { return _base_t::round_error(); }

    static constexpr me::IntegerWrapper<_Integer> infinity() noexcept
        { return _base_t::infinity(); }

    static constexpr me::IntegerWrapper<_Integer> quiet_NaN() noexcept
        { return _base_t::quiet_NaN(); }

    static constexpr me::IntegerWrapper<_Integer> signaling_NaN() noexcept
        { return _base_t::signaling_NaN(); }

    static constexpr me::IntegerWrapper<_Integer> denorm_min() noexcept
        { return _base_t::denorm_min(); }
};

using namespace me::literals;

// END

constexpr me::uint_fast8_t max_digit(std::uint_fast64_t __n) noexcept
{
    me::uint_fast8_t max = 0;

    for (; __n != 0; __n /= 10) {
        const me::uint_fast8_t d = __n % 10;

        if (d == 9) {
            return 9;
        } else if (d > max) {
            max = d;
        }
    }

    return max;
}

constexpr me::uint_fast8_t min_digit(std::uint_fast64_t __n) noexcept
{
    me::uint_fast8_t min = 9;

    for (; __n != 0; __n /= 10) {
        const me::uint_fast8_t d = __n % 10;

        if (d == 0) {
            return 0;
        } else if (d < min) {
            min = d;
        }
    }

    return min;
}

int main()
{
    std::ios_base::sync_with_stdio(false);
    std::cin.tie(nullptr);

    std::uint_fast16_t t;
    std::cin >> t;

    for (; t != 0; --t) {
        std::uint_fast64_t a, k;
        std::cin >> a >> k;

        for (; k != 1; --k) {
            if ((max_digit(a) == 0) || (min_digit(a) == 0)) {
                break;
            } else {
                a = a + max_digit(a) * min_digit(a);
            }
        }

        std::cout << a << '\n';
    }
}
