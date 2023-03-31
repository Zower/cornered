// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'books_generated.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$ContentType {
  String? get extraCss => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String? extraCss) html,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String? extraCss)? html,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String? extraCss)? html,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ContentType_Html value) html,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ContentType_Html value)? html,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ContentType_Html value)? html,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $ContentTypeCopyWith<ContentType> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ContentTypeCopyWith<$Res> {
  factory $ContentTypeCopyWith(
          ContentType value, $Res Function(ContentType) then) =
      _$ContentTypeCopyWithImpl<$Res, ContentType>;
  @useResult
  $Res call({String? extraCss});
}

/// @nodoc
class _$ContentTypeCopyWithImpl<$Res, $Val extends ContentType>
    implements $ContentTypeCopyWith<$Res> {
  _$ContentTypeCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? extraCss = freezed,
  }) {
    return _then(_value.copyWith(
      extraCss: freezed == extraCss
          ? _value.extraCss
          : extraCss // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$ContentType_HtmlCopyWith<$Res>
    implements $ContentTypeCopyWith<$Res> {
  factory _$$ContentType_HtmlCopyWith(
          _$ContentType_Html value, $Res Function(_$ContentType_Html) then) =
      __$$ContentType_HtmlCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String? extraCss});
}

/// @nodoc
class __$$ContentType_HtmlCopyWithImpl<$Res>
    extends _$ContentTypeCopyWithImpl<$Res, _$ContentType_Html>
    implements _$$ContentType_HtmlCopyWith<$Res> {
  __$$ContentType_HtmlCopyWithImpl(
      _$ContentType_Html _value, $Res Function(_$ContentType_Html) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? extraCss = freezed,
  }) {
    return _then(_$ContentType_Html(
      extraCss: freezed == extraCss
          ? _value.extraCss
          : extraCss // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc

class _$ContentType_Html implements ContentType_Html {
  const _$ContentType_Html({this.extraCss});

  @override
  final String? extraCss;

  @override
  String toString() {
    return 'ContentType.html(extraCss: $extraCss)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ContentType_Html &&
            (identical(other.extraCss, extraCss) ||
                other.extraCss == extraCss));
  }

  @override
  int get hashCode => Object.hash(runtimeType, extraCss);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ContentType_HtmlCopyWith<_$ContentType_Html> get copyWith =>
      __$$ContentType_HtmlCopyWithImpl<_$ContentType_Html>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String? extraCss) html,
  }) {
    return html(extraCss);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String? extraCss)? html,
  }) {
    return html?.call(extraCss);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String? extraCss)? html,
    required TResult orElse(),
  }) {
    if (html != null) {
      return html(extraCss);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ContentType_Html value) html,
  }) {
    return html(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ContentType_Html value)? html,
  }) {
    return html?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ContentType_Html value)? html,
    required TResult orElse(),
  }) {
    if (html != null) {
      return html(this);
    }
    return orElse();
  }
}

abstract class ContentType_Html implements ContentType {
  const factory ContentType_Html({final String? extraCss}) = _$ContentType_Html;

  @override
  String? get extraCss;
  @override
  @JsonKey(ignore: true)
  _$$ContentType_HtmlCopyWith<_$ContentType_Html> get copyWith =>
      throw _privateConstructorUsedError;
}
